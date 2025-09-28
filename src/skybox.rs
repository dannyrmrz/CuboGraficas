use crate::common::Vertex;

pub fn create_skybox_vertices() -> Vec<Vertex> {
    let mut vertices = Vec::new();
    
    // Crear un cubo grande para el skybox
    let size = 200.0;
    let positions = [
        // Front face
        [-size, -size, size], [size, -size, size], [-size, size, size],
        [-size, size, size], [size, -size, size], [size, size, size],
        // Right face
        [size, -size, size], [size, -size, -size], [size, size, size],
        [size, size, size], [size, -size, -size], [size, size, -size],
        // Back face
        [size, -size, -size], [-size, -size, -size], [size, size, -size],
        [size, size, -size], [-size, -size, -size], [-size, size, -size],
        // Left face
        [-size, -size, -size], [-size, -size, size], [-size, size, -size],
        [-size, size, -size], [-size, -size, size], [-size, size, size],
        // Top face
        [-size, size, size], [size, size, size], [-size, size, -size],
        [-size, size, -size], [size, size, size], [size, size, -size],
        // Bottom face
        [-size, -size, -size], [size, -size, -size], [-size, -size, size],
        [-size, -size, size], [size, -size, -size], [size, -size, size],
    ];

    // Normales apuntando hacia adentro
    let normals = [
        // Front face
        [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0],
        // Right face
        [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0],
        // Back face
        [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0],
        // Left face
        [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0],
        // Top face
        [0.0, -1.0, 0.0], [0.0, -1.0, 0.0], [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0], [0.0, -1.0, 0.0], [0.0, -1.0, 0.0],
        // Bottom face
        [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0],
    ];

    // Coordenadas UV para el skybox (cada cara usa la misma textura)
    let tex_coords = [
        // Front face
        [0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0],
        // Right face
        [0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0],
        // Back face
        [0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0],
        // Left face
        [0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0],
        // Top face
        [0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0],
        // Bottom face
        [0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0],
    ];

    for i in 0..positions.len() {
        vertices.push(Vertex {
            position: [positions[i][0], positions[i][1], positions[i][2], 1.0],
            normal: [normals[i][0], normals[i][1], normals[i][2], 1.0],
            tex_coords: tex_coords[i],
            material_id: 5.0, // Skybox usa material especial (índice 5)
        });
    }

    vertices
}
