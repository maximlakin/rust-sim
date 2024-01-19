struct CameraUniform {
    view_position: vec4<f32>,
    view_proj: mat4x4<f32>,
}
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
}
struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) world_position: vec3<f32>,
}

@vertex
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    var out: VertexOutput;
    var world_position: vec4<f32> = model_matrix * vec4<f32>(model.position, 1.0);
    out.world_position = world_position.xyz;
    out.clip_position = camera.view_proj * world_position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let light_pos = vec3<f32>(0.0, 0.0, 6.0);

    let world_normal = vec3<f32>(0.0, 0.0, 1.0);

    let light_dir = normalize(light_pos - in.world_position);

    let view_dir = normalize(camera.view_position.xyz - in.world_position);
    let half_dir = normalize(view_dir + light_dir);

    let specular_strength = pow(max(dot(world_normal, half_dir), 0.0), 16.0);
    let specular_color = specular_strength * vec3<f32>(1.0, 1.0, 1.0);

    let diffuse_strength = max(dot(world_normal, light_dir), 0.0);
    let diffuse_color = vec3<f32>(1.0, 1.0, 1.0) * diffuse_strength;

    let object_color = vec4<f32>(0.4, 0.45, 0.5, 1.0);
    let result = ( 0.1*diffuse_color + specular_color ) * object_color.xyz;

    return vec4<f32>(result, object_color.a);
}