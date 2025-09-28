struct Uniforms {
    model_mat : mat4x4<f32>,
    view_project_mat : mat4x4<f32>,
    normal_mat : mat4x4<f32>,
};

@binding(0) @group(0) var<uniform> uniforms : Uniforms;

struct Output {
    @builtin(position) position : vec4<f32>,
    @location(0) v_position : vec4<f32>,
    @location(1) v_normal : vec4<f32>,
    @location(2) v_tex_coords : vec2<f32>,
    @location(3) v_material_id : f32,
};

@vertex
fn vs_main(@location(0) pos: vec4<f32>, @location(1) normal: vec4<f32>, @location(2) tex_coords: vec2<f32>, @location(3) material_id: f32) -> Output {
    var output: Output;
    let m_position:vec4<f32> = uniforms.model_mat * pos; 
    output.v_position = m_position;
    output.v_normal =  uniforms.normal_mat * normal;
    output.v_tex_coords = tex_coords;
    output.v_material_id = material_id;
    output.position = uniforms.view_project_mat * m_position;               
    return output;
}


struct FragUniforms {
    light_position : vec4<f32>,
    eye_position : vec4<f32>,
};
@binding(1) @group(0) var<uniform> frag_uniforms : FragUniforms;

struct LightUniforms {
    color : vec4<f32>,
    specular_color : vec4<f32>,
    ambient_intensity: f32,
    diffuse_intensity :f32,
    specular_intensity: f32,
    specular_shininess: f32,
};
@binding(2) @group(0) var<uniform> light_uniforms : LightUniforms;

@binding(3) @group(0) var texture_array: texture_2d_array<f32>;
@binding(4) @group(0) var texture_sampler: sampler;

struct Material {
    albedo: vec3<f32>,
    specular: f32,
    transparency: f32,
    reflectivity: f32,
    refractive_index: f32,
    texture_id: u32,
};

@binding(5) @group(0) var<storage, read> materials: array<Material>;

@fragment
fn fs_main(@location(0) v_position: vec4<f32>, @location(1) v_normal: vec4<f32>, @location(2) v_tex_coords: vec2<f32>, @location(3) v_material_id: f32) ->  @location(0) vec4<f32> {
    let N:vec3<f32> = normalize(v_normal.xyz);
    let L:vec3<f32> = normalize(frag_uniforms.light_position.xyz - v_position.xyz);
    let V:vec3<f32> = normalize(frag_uniforms.eye_position.xyz - v_position.xyz);
    let H:vec3<f32> = normalize(L + V);
    
    // Get material properties based on material ID
    let material = materials[i32(v_material_id)];
    
    // Sample texture based on material
    let texture_color = textureSample(texture_array, texture_sampler, v_tex_coords, u32(material.texture_id));
    
    // Calculate lighting
    let diffuse:f32 = max(dot(N, L), 0.0);
    let specular: f32 = pow(max(dot(N, H), 0.0), light_uniforms.specular_shininess);
    let ambient:f32 = light_uniforms.ambient_intensity;
    
    // Apply material properties
    let base_color = texture_color.rgb;
    let diffuse_color = base_color * light_uniforms.diffuse_intensity * diffuse;
    let specular_color = light_uniforms.specular_color.rgb * light_uniforms.specular_intensity * specular * material.specular;
    let ambient_color = base_color * ambient;
    
    let lit_color = ambient_color + diffuse_color + specular_color;
    
    // Apply transparency
    let alpha = texture_color.a * (1.0 - material.transparency);
    
    // Para cristal, hacer más transparente
    let final_alpha = select(alpha, 0.3, material.transparency > 0.5);
    
    // Para skybox, usar alpha completo
    let skybox_alpha = select(final_alpha, 1.0, v_material_id == 5.0);
    
    let final_color = vec4<f32>(lit_color, skybox_alpha);
    
    return final_color;
}
