use std::{fs::File, io::Write};

use rmesh::{write_rmesh, RMeshError, Header, ComplexMesh, Texture, Vertex};

pub const ROOM_SCALE: f32 = 8. / 2048.;

fn main() -> Result<(), RMeshError> {
    let min_x = -1.0 / ROOM_SCALE;
    let min_y = -1.0 / ROOM_SCALE;
    let min_z = -1.0 / ROOM_SCALE;
    let max_x = 1.0 / ROOM_SCALE;
    let max_y = 1.0 / ROOM_SCALE;
    let max_z = 1.0 / ROOM_SCALE;
    let header = Header {
        meshes: vec![
            ComplexMesh {
                textures: [Texture::empty(), Texture::empty()],
                vertices: vec![
                    // Front
                    Vertex {
                        position: [min_x, min_y, max_z],
                        tex_coords: [
                            [0., 0.], // Texture
                            [0., 0.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [max_x, min_y, max_z],
                        tex_coords: [
                            [1., 0.], // Texture
                            [1., 0.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [max_x, max_y, max_z],
                        tex_coords: [
                            [1., 1.], // Texture
                            [1., 1.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [min_x, max_y, max_z],
                        tex_coords: [
                            [0., 1.], // Texture
                            [0., 1.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    // Back
                    Vertex {
                        position: [min_x, max_y, min_z],
                        tex_coords: [
                            [1., 0.], // Texture
                            [1., 0.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [max_x, max_y, min_z],
                        tex_coords: [
                            [0., 0.], // Texture
                            [0., 0.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [max_x, min_y, min_z],
                        tex_coords: [
                            [0., 1.], // Texture
                            [0., 1.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [min_x, min_y, min_z],
                        tex_coords: [
                            [1., 1.], // Texture
                            [1., 1.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    // Right
                    Vertex {
                        position: [max_x, min_y, min_z],
                        tex_coords: [
                            [0., 0.], // Texture
                            [0., 0.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [max_x, max_y, min_z],
                        tex_coords: [
                            [1., 0.], // Texture
                            [1., 0.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [max_x, max_y, max_z],
                        tex_coords: [
                            [1., 1.], // Texture
                            [1., 1.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [max_x, min_y, max_z],
                        tex_coords: [
                            [0., 1.], // Texture
                            [0., 1.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    // Left
                    Vertex {
                        position: [min_x, min_y, max_z],
                        tex_coords: [
                            [1., 0.], // Texture
                            [1., 0.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [min_x, max_y, max_z],
                        tex_coords: [
                            [0., 0.], // Texture
                            [0., 0.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [min_x, max_y, min_z],
                        tex_coords: [
                            [0., 1.], // Texture
                            [0., 1.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [min_x, min_y, min_z],
                        tex_coords: [
                            [1., 1.], // Texture
                            [1., 1.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    // Top
                    Vertex {
                        position: [max_x, max_y, min_z],
                        tex_coords: [
                            [1., 0.], // Texture
                            [1., 0.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [min_x, max_y, min_z],
                        tex_coords: [
                            [0., 0.], // Texture
                            [0., 0.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [min_x, max_y, max_z],
                        tex_coords: [
                            [0., 1.], // Texture
                            [0., 1.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [max_x, max_y, max_z],
                        tex_coords: [
                            [1., 1.], // Texture
                            [1., 1.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    // Bottom
                    Vertex {
                        position: [max_x, min_y, max_z],
                        tex_coords: [
                            [0., 0.], // Texture
                            [0., 0.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [min_x, min_y, max_z],
                        tex_coords: [
                            [1., 0.], // Texture
                            [1., 0.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [min_x, min_y, min_z],
                        tex_coords: [
                            [1., 1.], // Texture
                            [1., 1.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                    Vertex {
                        position: [max_x, min_y, min_z],
                        tex_coords: [
                            [0., 1.], // Texture
                            [0., 1.], // Lightmap
                        ],
                        color: [0, 0, 0],
                    },
                ],
                triangles: vec![
                    [0, 1, 2], [2, 3, 0], // Front
                    [4, 5, 6], [6, 7, 4], // Back
                    [8, 9, 10], [10, 11, 8], // Right
                    [12, 13, 14], [14, 15, 12], // Left
                    [16, 17, 18], [18, 19, 16], // Top
                    [20, 21, 22], [22, 23, 20], // Bottom
                ],
            }
        ],
        colliders: vec![],
        trigger_boxes: vec![],
        entities: vec![],
    };
    let rmesh = write_rmesh(&header)?;
    let mut file = File::create("assets/cube.rmesh").unwrap();
    file.write_all(&rmesh).unwrap();
    Ok(())
}
