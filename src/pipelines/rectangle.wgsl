// vertex and fragment shader

struct InstanceInput {
    @location(1) position: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) color: vec3<f32>,
}

struct VertexInput {
    @location(0) position: vec3<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(3) color: vec3<f32>
};

@vertex
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(
        model.position.x * instance.size.x + instance.position.x + instance.size.x,
        model.position.y * instance.size.y + instance.position.y - instance.size.y,
        0.0,
        1.0
    );
    out.color = instance.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}