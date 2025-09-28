use crate::materials::{Block, Material};

pub struct Diorama {
    pub blocks: Vec<Block>,
    pub materials: Vec<Material>,
}

impl Diorama {
    pub fn new() -> Self {
        let mut blocks = Vec::new();
        let materials = vec![
            Material::grass_block(),    // 0
            Material::stone(),          // 1
            Material::water(),          // 2
            Material::glass(),          // 3
            Material::diamond_block(),  // 4
        ];

        // Crear la isla base de Skyblock
        Self::create_skyblock_island(&mut blocks);

        Diorama { blocks, materials }
    }

    fn create_skyblock_island(blocks: &mut Vec<Block>) {
        // Base de la isla (piedra)
        for x in 0..5 {
            for z in 0..5 {
                blocks.push(Block::new(x as f32, 0.0, z as f32, 1)); // Piedra
            }
        }

        // Capa superior (hierba)
        for x in 1..4 {
            for z in 1..4 {
                blocks.push(Block::new(x as f32, 1.0, z as f32, 0)); // Hierba
            }
        }

        // Fuente de agua en el centro
        blocks.push(Block::new(2.0, 2.0, 2.0, 2)); // Agua

        // Invernadero de cristal
        for x in 0..3 {
            for z in 0..3 {
                if x == 1 && z == 1 {
                    continue; // Espacio para el agua
                }
                blocks.push(Block::new(x as f32 + 6.0, 1.0, z as f32, 3)); // Cristal
                blocks.push(Block::new(x as f32 + 6.0, 2.0, z as f32, 3)); // Cristal
            }
        }

        // Tesoro de diamantes
        blocks.push(Block::new(8.0, 1.0, 8.0, 4)); // Diamante
        blocks.push(Block::new(8.0, 2.0, 8.0, 4)); // Diamante
        blocks.push(Block::new(8.0, 3.0, 8.0, 4)); // Diamante

        // Árbol (tronco de piedra, hojas de hierba)
        blocks.push(Block::new(0.0, 2.0, 0.0, 1)); // Tronco
        blocks.push(Block::new(0.0, 3.0, 0.0, 1)); // Tronco
        blocks.push(Block::new(-1.0, 3.0, 0.0, 0)); // Hoja
        blocks.push(Block::new(1.0, 3.0, 0.0, 0));  // Hoja
        blocks.push(Block::new(0.0, 3.0, -1.0, 0)); // Hoja
        blocks.push(Block::new(0.0, 3.0, 1.0, 0));  // Hoja
        blocks.push(Block::new(0.0, 4.0, 0.0, 0));  // Hoja superior
    }

    pub fn get_vertex_data(&self) -> Vec<crate::common::Vertex> {
        let mut vertices = Vec::new();
        
        for block in &self.blocks {
            let block_vertices = self.create_cube_vertices(block.position, block.material_id);
            vertices.extend(block_vertices);
        }
        
        vertices
    }

    fn create_cube_vertices(&self, position: [f32; 3], _material_id: u32) -> Vec<crate::common::Vertex> {
        let [x, y, z] = position;
        let mut vertices = Vec::new();

        // Posiciones de los vértices del cubo
        let positions = [
            // Front face
            [x-0.5, y-0.5, z+0.5], [x+0.5, y-0.5, z+0.5], [x-0.5, y+0.5, z+0.5],
            [x-0.5, y+0.5, z+0.5], [x+0.5, y-0.5, z+0.5], [x+0.5, y+0.5, z+0.5],
            // Right face
            [x+0.5, y-0.5, z+0.5], [x+0.5, y-0.5, z-0.5], [x+0.5, y+0.5, z+0.5],
            [x+0.5, y+0.5, z+0.5], [x+0.5, y-0.5, z-0.5], [x+0.5, y+0.5, z-0.5],
            // Back face
            [x+0.5, y-0.5, z-0.5], [x-0.5, y-0.5, z-0.5], [x+0.5, y+0.5, z-0.5],
            [x+0.5, y+0.5, z-0.5], [x-0.5, y-0.5, z-0.5], [x-0.5, y+0.5, z-0.5],
            // Left face
            [x-0.5, y-0.5, z-0.5], [x-0.5, y-0.5, z+0.5], [x-0.5, y+0.5, z-0.5],
            [x-0.5, y+0.5, z-0.5], [x-0.5, y-0.5, z+0.5], [x-0.5, y+0.5, z+0.5],
            // Top face
            [x-0.5, y+0.5, z+0.5], [x+0.5, y+0.5, z+0.5], [x-0.5, y+0.5, z-0.5],
            [x-0.5, y+0.5, z-0.5], [x+0.5, y+0.5, z+0.5], [x+0.5, y+0.5, z-0.5],
            // Bottom face
            [x-0.5, y-0.5, z-0.5], [x+0.5, y-0.5, z-0.5], [x-0.5, y-0.5, z+0.5],
            [x-0.5, y-0.5, z+0.5], [x+0.5, y-0.5, z-0.5], [x+0.5, y-0.5, z+0.5],
        ];

        // Normales para cada cara
        let normals = [
            // Front face
            [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0],
            // Right face
            [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0],
            // Back face
            [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0],
            // Left face
            [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0],
            // Top face
            [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0],
            // Bottom face
            [0.0, -1.0, 0.0], [0.0, -1.0, 0.0], [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0], [0.0, -1.0, 0.0], [0.0, -1.0, 0.0],
        ];

        // Coordenadas UV
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
            vertices.push(crate::common::Vertex {
                position: [positions[i][0], positions[i][1], positions[i][2], 1.0],
                normal: [normals[i][0], normals[i][1], normals[i][2], 1.0],
                tex_coords: tex_coords[i],
                material_id: _material_id as f32,
            });
        }

        vertices
    }
}
