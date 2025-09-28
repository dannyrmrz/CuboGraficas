mod common;
mod vertex_data;
mod transforms;
mod materials;
mod diorama;
mod skybox;

fn vertex(p:[i8; 3], n: [i8; 3], uv: [f32; 2], material_id: f32) -> common::Vertex {
    common::Vertex {
        position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
        normal: [n[0] as f32, n[1] as f32, n[2] as f32, 1.0],
        tex_coords: uv,
        material_id,
    }
}

fn create_vertices() -> Vec<common::Vertex> {
    let pos = vertex_data::cube_positions();
    let normal= vertex_data::cube_normals();
    let tex_coords = vertex_data::cube_tex_coords();
    
    let mut data:Vec<common::Vertex> = Vec::with_capacity(pos.len());
    for i in 0..pos.len() {
        data.push(vertex(pos[i], normal[i], tex_coords[i], 0.0)); // Material 0 por defecto
    }
    data.to_vec()
}

fn main(){
    let diorama = diorama::Diorama::new();
    let mut vertex_data = diorama.get_vertex_data();
    
    // Agregar skybox
    let skybox_vertices = skybox::create_skybox_vertices();
    vertex_data.extend(skybox_vertices);
    
    let light_data = common::light([1.0,1.0,1.0], [1.0, 1.0, 1.0], 0.2, 0.7, 0.3, 16.0);
    common::run(&vertex_data, light_data, "Skyblock Diorama");
}