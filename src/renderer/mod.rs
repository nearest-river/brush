
pub mod pipeline;

use crate::prelude::*;
use pipeline::PipelineBuilder;

use std::{
  iter,
  sync::Arc,
};

use winit::{
  window::Window,
  dpi::PhysicalSize,
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


pub struct Renderer {
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
  pub async fn new(window: Arc<Window>,clear_color: Color)-> Self {
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

    let render_pipeline=PipelineBuilder::new_with_shader(SHADER_SOURCE,"vs_main","fs_main")
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

  pub(crate) fn render(&self) {
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

  pub(crate) fn resize(&mut self,size: PhysicalSize<u32>) {
    if size.width==0 || size.height==0 {
      return;
    }

    self.surface_config.width=size.width;
    self.surface_config.height=size.height;

    self.surface.configure(&self.dev,&self.surface_config);
  }
}




