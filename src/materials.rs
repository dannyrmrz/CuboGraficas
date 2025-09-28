use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Material {
    pub albedo: [f32; 3],        // Color base del material
    pub specular: f32,           // Intensidad especular
    pub transparency: f32,       // Transparencia (0 = opaco, 1 = transparente)
    pub reflectivity: f32,       // Reflexión (0 = no refleja, 1 = espejo)
    pub refractive_index: f32,   // Índice de refracción
    pub texture_id: u32,         // ID de la textura (0-4)
}

impl Material {
    pub fn grass_block() -> Self {
        Material {
            albedo: [0.2, 0.8, 0.2],  // Verde
            specular: 0.1,
            transparency: 0.0,
            reflectivity: 0.0,
            refractive_index: 1.0,
            texture_id: 0,
        }
    }

    pub fn stone() -> Self {
        Material {
            albedo: [0.5, 0.5, 0.5],  // Gris
            specular: 0.3,
            transparency: 0.0,
            reflectivity: 0.0,
            refractive_index: 1.0,
            texture_id: 1,
        }
    }

    pub fn water() -> Self {
        Material {
            albedo: [0.1, 0.3, 0.8],  // Azul
            specular: 0.6,
            transparency: 0.6,
            reflectivity: 0.3,
            refractive_index: 1.33,    // Agua
            texture_id: 2,
        }
    }

    pub fn glass() -> Self {
        Material {
            albedo: [0.9, 0.9, 0.9],  // Blanco transparente
            specular: 0.9,
            transparency: 0.8,
            reflectivity: 0.2,
            refractive_index: 1.5,     // Vidrio
            texture_id: 3,
        }
    }

    pub fn diamond_block() -> Self {
        Material {
            albedo: [0.2, 0.6, 1.0],  // Azul brillante
            specular: 0.95,
            transparency: 0.0,
            reflectivity: 0.8,
            refractive_index: 1.0,
            texture_id: 4,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Block {
    pub position: [f32; 3],
    pub material_id: u32,
}

impl Block {
    pub fn new(x: f32, y: f32, z: f32, material_id: u32) -> Self {
        Block {
            position: [x, y, z],
            material_id,
        }
    }
}
