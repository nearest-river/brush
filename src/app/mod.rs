
mod pipeline;

use crate::prelude::*;
pub use pipeline::PipelineBuilder;

use std::{
  mem,
  iter,
  sync::Arc,
};

use winit::{
  dpi::PhysicalSize,
  event::WindowEvent,
  application::ApplicationHandler,
  window::{
    Window,
    WindowId,
  },
  event_loop::{
    EventLoop,
    ActiveEventLoop,
  }
};

use wgpu::{
  Queue,
  Device,
  LoadOp,
  StoreOp,
  Surface,
  Instance,
  Operations,
  RenderPipeline,
  PowerPreference,
  DeviceDescriptor,
  RenderPassDescriptor,
  SurfaceConfiguration,
  CurrentSurfaceTexture,
  TextureViewDescriptor,
  CommandEncoderDescriptor,
  RenderPassColorAttachment,
  RequestAdapterOptionsBase,
};


pub struct App {
  state: AppState,
  event_loop: EventLoop<()>
}

impl App {
  #[inline]
  pub fn new()-> Self {
    let event_loop=EventLoop::new()
    .unwrap();

    let state=AppState::default();

    Self {
      state,
      event_loop,
    }
  }

  #[inline(always)]
  pub fn with_title(mut self,title: impl Into<String>)-> Self {
    self.state.title=title.into();
    self
  }

  #[inline(always)]
  pub fn with_clear_color(mut self,color: impl IntoColor)-> Self {
    self.state.clear_color=color.into_color();
    self
  }

  #[inline(always)]
  pub fn with_inner_size(mut self,inner_size: PhysicalSize<u32>)-> Self {
    self.state.inner_size=inner_size;
    self
  }

  #[inline]
  pub fn run(mut self) {
    self.event_loop.run_app(&mut self.state)
    .unwrap();
  }
}

struct AppState {
  title: String,
  clear_color: Color,
  inner_size: PhysicalSize<u32>,
  renderer: Option<Renderer>,
  window: Option<Arc<Window>>,
}

impl ApplicationHandler for AppState {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    if self.window.is_some() {
      return;
    }

    let window=Arc::new(
      event_loop.create_window(
        Window::default_attributes()
        .with_title(mem::take(&mut self.title))
        .with_inner_size(self.inner_size),
      ).expect("failed to create window"),
    );

    let state=pollster::block_on(Renderer::new(window.clone(),self.clear_color));

    self.window=Some(window);
    self.renderer=Some(state);
  }

  fn window_event(&mut self,event_loop: &ActiveEventLoop,window_id: WindowId,event: WindowEvent) {
    let Some(window)=&self.window else {
      return;
    };

    if window.id()!=window_id {
      return;
    }

    match event {
      WindowEvent::CloseRequested=> event_loop.exit(),
      WindowEvent::Resized(size)=> {
        if let Some(renderer)=&mut self.renderer {
          renderer.resize(size);
        }
      },
      WindowEvent::RedrawRequested=> {
        if let Some(renderer)=&mut self.renderer {
          renderer.render();
        }
      },
      _=> {},
    }
  }

  fn about_to_wait(&mut self,_event_loop: &ActiveEventLoop) {
    if let Some(window)=&self.window {
      window.request_redraw();
    }
  }
}

struct Renderer {
  dev: Device,
  queue: Queue,
  clear_color: Color,
  window: Arc<Window>,
  surface: Surface<'static>,
  render_pipeline: RenderPipeline,
  surface_config: SurfaceConfiguration,
}

static SHADER_SOURCE: &str=include_str!("../shaders/hello_world.wgsl");


impl Renderer {
  async fn new(window: Arc<Window>,clear_color: Color)-> Self {
    let inner_size=window.inner_size();
    let instance=Instance::default();

    let surface=instance.create_surface(window.clone())
    .expect("failed to create surface");

    let adapter_descriptor=RequestAdapterOptionsBase {
      power_preference: PowerPreference::HighPerformance,
      compatible_surface: Some(&surface),
      ..Default::default()
    };
    let adapter=instance.request_adapter(&adapter_descriptor)
    .await
    .expect("no compatible GPU adapter");

    let dev_descriptor=DeviceDescriptor {
      label: Some("device"),
      ..Default::default()
    };
    let (dev,queue)=adapter.request_device(&dev_descriptor)
    .await
    .expect("failed to request a device");

    let surface_config=surface.get_default_config(&adapter, inner_size.width, inner_size.height)
    .expect("surface is incompatible with adapter");
    surface.configure(&dev,&surface_config);

    let render_pipeline=PipelineBuilder::new()
    .with_shader(SHADER_SOURCE,"vs_main","fs_main")
    .with_pixel_format(surface_config.format)
    .build(dev.clone());

    Self {
      dev,
      queue,
      window,
      surface,
      clear_color,
      surface_config,
      render_pipeline,
    }
  }

  fn render(&self) {
    let surface_texture=match self.surface.get_current_texture() {
      CurrentSurfaceTexture::Success(frame)=> frame,
      CurrentSurfaceTexture::Suboptimal(frame)=> {
        self.surface.configure(&self.dev,&self.surface_config);
        frame
      }
      CurrentSurfaceTexture::Lost|CurrentSurfaceTexture::Outdated=> {
        self.surface.configure(&self.dev,&self.surface_config);
        return;
      }
      CurrentSurfaceTexture::Timeout|CurrentSurfaceTexture::Occluded|CurrentSurfaceTexture::Validation=> return,
    };

    let texture_view_desc=TextureViewDescriptor::default();
    let texture_view=surface_texture.texture.create_view(&texture_view_desc);

    let command_encoder_desc=CommandEncoderDescriptor {
      label: Some("render encoder"),
    };

    let mut command_encoder=self.dev.create_command_encoder(&command_encoder_desc);

    let color_attachment=RenderPassColorAttachment {
      view: &texture_view,
      ops: Operations {
        store: StoreOp::Store,
        load: LoadOp::Clear(self.clear_color.into_wgpu_color()),
      },
      resolve_target: None,
      depth_slice: None,
    };

    {
      let render_pass_desc=RenderPassDescriptor {
        label: Some("render pass"),
        color_attachments: &[Some(color_attachment)],
        ..Default::default()
      };

      let mut render_pass=command_encoder.begin_render_pass(&render_pass_desc);
      render_pass.set_pipeline(&self.render_pipeline);
      render_pass.draw(0..3,0..1);
    }

    self.queue.submit(iter::once(command_encoder.finish()));

    self.window.pre_present_notify();
    self.queue.present(surface_texture);
  }

  fn resize(&mut self,size: PhysicalSize<u32>) {
    if size.width==0 || size.height==0 {
      return;
    }

    self.surface_config.width=size.width;
    self.surface_config.height=size.height;

    self.surface.configure(&self.dev,&self.surface_config);
  }
}

impl Default for AppState {
  #[inline(always)]
  fn default()-> Self {
    Self {
      window: None,
      renderer: None,
      title: "brush window".to_owned(),
      clear_color: Color::default(),
      inner_size: PhysicalSize::new(1280, 720),
    }
  }
}






