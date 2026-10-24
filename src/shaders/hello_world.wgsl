
struct VertexPayload {
  @builtin(position) pos: vec4<f32>,
  @location(0) color: vec4<f32>,
}


@vertex
fn vs_main(@builtin(vertex_index) i: u32)-> VertexPayload {
  let positions=array<vec2<f32>,3>(
    vec2<f32>(-0.6,0.0),
    vec2<f32>(0.6,0.0),
    vec2<f32>(0.0,0.6),
  );

  let colors=array<vec4<f32>,3>(
    vec4<f32>(1,0,0,1),
    vec4<f32>(0,1,0,1),
    vec4<f32>(0,0,1,1),
  );

  var out: VertexPayload;
  out.pos=vec4(positions[i],0,1);
  out.color=colors[i];

  return out;
}

@fragment
fn fs_main(in: VertexPayload)-> @location(0) vec4<f32> {
  return in.color;
}











