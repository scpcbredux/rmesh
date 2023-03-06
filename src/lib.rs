#![warn(missing_docs)]

mod error;
pub use crate::error::*;

use std::io::{Cursor, Read, Seek};
use byteorder::{ReadBytesExt, LittleEndian};

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
    /// Example
    /// ```rust
    /// let bytes = unimplemented!();
    ///
    /// let rmesh = rmesh::RMesh::read(&bytes)?;
    ///
    /// let positions: Vec<_> = rmesh.meshes[0].vertices.iter().map(|v| v.position).collect();
    /// let tex_coords: Vec<_> = rmesh.meshes[0].vertices.iter().map(|v| v.tex_coords).collect();
    ///
    /// println!("Postions: {:#?}", positions);
    /// println!("UVS: {:#?}", tex_coords);
    /// ```
    pub fn read(data: &[u8]) -> Result<Self, RMeshError> {
        let mut cursor = Cursor::new(data);

        let tag = Self::read_fixed_length_string(&mut cursor)?;

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
                    let name = Self::read_fixed_length_string(&mut cursor)?;
                    textures.push(name);
                } else {
                    cursor.read_u32::<LittleEndian>()?;
                }
            }

            let vertex_count = cursor.read_u32::<LittleEndian>()?;

            for _ in 0..vertex_count {
                let mut position = [0.0; 3];
                cursor.read_f32_into::<LittleEndian>(&mut position)?;

                cursor.read_f32::<LittleEndian>()?; // Unused
                cursor.read_f32::<LittleEndian>()?; // Unused

                let mut tex_coords = [0.0; 2];
                cursor.read_f32_into::<LittleEndian>(&mut tex_coords)?;
                
                let mut color = [0; 3];
                cursor.read_exact(&mut color)?;

                vertices.push(Vertex {
                    position,
                    tex_coords,
                    color,
                });
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

    fn read_fixed_length_string<T>(data: &mut T) -> Result<String, RMeshError>
    where
        T: Read + Seek
    {
        let len = data.read_u32::<LittleEndian>()?;
        let mut buf = vec![0; len as usize];
        data.read_exact(&mut buf)?;
        Ok(String::from_utf8(buf)?)
    }
}
