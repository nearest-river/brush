

use crate::{
  prelude::*,
  renderer::Renderer,
};

use std::{
  mem,
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






