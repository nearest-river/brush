
use std::borrow::Cow;
use wgpu::{
  Face,
  Device,
  FrontFace,
  BlendState,
  ColorWrites,
  PolygonMode,
  VertexState,
  ShaderSource,
  FragmentState,
  TextureFormat,
  PrimitiveState,
  RenderPipeline,
  ColorTargetState,
  MultisampleState,
  PrimitiveTopology,
  ShaderModuleDescriptor,
  PipelineLayoutDescriptor,
  RenderPipelineDescriptor,
  PipelineCompilationOptions,
};



#[derive(Debug)]
pub struct PipelineBuilder<'a> {
  shader_source: Option<&'a str>,
  vertex_entry: Option<&'a str>,
  fragment_entry: Option<&'a str>,
  px_fmt: TextureFormat,
}



impl<'a> PipelineBuilder<'a> {
  #[inline(always)]
  pub const fn new()-> Self {
    Self {
      fragment_entry: None,
      vertex_entry: None,
      shader_source: None,
      px_fmt: TextureFormat::Rgba8Unorm,
    }
  }

  #[inline(always)]
  pub const fn new_with_shader(shader_source: &'a str,vertex_entry: &'a str,fragment_entry: &'a str)-> Self {
    let mut this=Self::new();

    this.with_shader(shader_source, vertex_entry, fragment_entry);
    this
  }

  #[inline(always)]
  pub const fn with_shader(&mut self,shader_source: &'a str,vertex_entry: &'a str,fragment_entry: &'a str)-> &mut Self {
    self.shader_source=Some(shader_source);
    self.vertex_entry=Some(vertex_entry);
    self.fragment_entry=Some(fragment_entry);

    self
  }

  #[inline(always)]
  pub const fn with_pixel_format(&mut self,px_fmt: TextureFormat)-> &mut Self {
    self.px_fmt=px_fmt;
    self
  }


  pub fn build(&self,dev: Device)-> RenderPipeline {
    let shader_source=self.shader_source.expect("no shader source provided");

    let shader_module_desc=ShaderModuleDescriptor {
      label: Some("shader module"),
      source: ShaderSource::Wgsl(Cow::Borrowed(shader_source)),
    };
    let shader_module=dev.create_shader_module(shader_module_desc);

    let pipeline_layout_desc=PipelineLayoutDescriptor {
      label: Some("render pipeline layout"),
      ..Default::default()
    };
    let pipeline_layout=dev.create_pipeline_layout(&pipeline_layout_desc);

    let render_targets=[Some(ColorTargetState {
      format: self.px_fmt,
      write_mask: ColorWrites::ALL,
      blend: Some(BlendState::REPLACE),
    })];

    let render_pipeline_desc=RenderPipelineDescriptor {
      cache: None,
      depth_stencil: None,
      multiview_mask: None,
      label: Some("render pipeline"),
      layout: Some(&pipeline_layout),
      multisample: MultisampleState {
        mask: !0,
        count: 1,
        alpha_to_coverage_enabled: false,
      },
      vertex: VertexState {
        buffers: &[],
        module: &shader_module,
        entry_point: self.vertex_entry,
        compilation_options: PipelineCompilationOptions::default(),
      },
      fragment: Some(FragmentState {
        module: &shader_module,
        targets: &render_targets,
        entry_point: self.fragment_entry,
        compilation_options: PipelineCompilationOptions::default(),
      }),
      primitive: PrimitiveState {
        topology: PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: FrontFace::Ccw,
        cull_mode: Some(Face::Back),
        polygon_mode: PolygonMode::Fill,
        unclipped_depth: false,
        conservative: false,
      },
    };

    dev.create_render_pipeline(&render_pipeline_desc)
  }
}


impl Default for PipelineBuilder<'_> {
  #[inline(always)]
  fn default()-> Self {
    Self::new()
  }
}




























