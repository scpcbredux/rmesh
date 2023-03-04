use std::io::{Cursor, Read, Seek};
use byteorder::{ReadBytesExt, LittleEndian};
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RMeshError {
    #[error("Invalid Header: (Expected RoomMesh or RoomMesh.HasTriggerBox, instead got {0})")]
    InvalidHeader(String),
}

pub fn read_fixed_length_string<T>(data: &mut T, len: usize) -> Result<String> 
where
    T: Read + Seek
{
    let mut tag_buf = vec![0; len];
    data.read_exact(&mut tag_buf)?;
    Ok(String::from_utf8(tag_buf)?)
}

#[derive(Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub color: [u8; 3],
}

#[derive(Debug)]
pub struct Mesh {
    pub textures: Vec<String>,
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<[u32; 3]>,
}

#[derive(Debug)]
pub struct RMesh {
    pub tag: String,
    pub meshes: Vec<Mesh>,
}

impl RMesh {
    pub fn read(data: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(data);

        let header_len = cursor.read_u32::<LittleEndian>()?;
        let tag = read_fixed_length_string(&mut cursor, header_len as usize)?;

        if tag != "RoomMesh" && tag != "RoomMesh.HasTriggerBox" {
            return Err(RMeshError::InvalidHeader(tag).into())
        }

        let mesh_count = cursor.read_u32::<LittleEndian>()?;
        let mut meshes = vec![];

        for _ in 0..mesh_count {
            let mut textures = vec![];
            let mut vertices = vec![];
            let mut triangles = vec![];

            for _ in 0..2 {
                let alpha_type = cursor.read_u8()?;
                if alpha_type != 0 {
                    let name_len = cursor.read_u32::<LittleEndian>()?;
                    let name = read_fixed_length_string(&mut cursor, name_len as usize)?;
                    textures.push(name);
                } else {
                    cursor.read_u32::<LittleEndian>()?;
                }
            }

            let vertex_count = cursor.read_u32::<LittleEndian>()?;

            for _ in 0..vertex_count {
                let mut position = [0.0; 3];
                cursor.read_f32_into::<LittleEndian>(&mut position)?;

                let mut tex_coords = [0.0; 2];
                cursor.read_f32_into::<LittleEndian>(&mut tex_coords)?;

                cursor.read_f32::<LittleEndian>()?; // Unused
                cursor.read_f32::<LittleEndian>()?; // Unused
                
                let mut color = [0; 3];
                cursor.read_exact(&mut color)?;

                let vertex = Vertex {
                    position,
                    tex_coords,
                    color,
                };
                vertices.push(vertex);
            }

            let poly_count = cursor.read_u32::<LittleEndian>()?;

            for _ in 0..poly_count {
                let mut tri = [0; 3];
                cursor.read_u32_into::<LittleEndian>(&mut tri)?;

                triangles.push(tri);
            }

            meshes.push(Mesh {
                textures,
                vertices,
                triangles,
            });
        }

        Ok(Self {
            tag,
            meshes,
        })
    }
}
