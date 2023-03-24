#![warn(missing_docs)]

//! # RMesh
//! 
//! `rmesh` is a library for readering and parsering for room meshes that use the rmesh extension found in scpcb.

use std::io::{Cursor, Read, Seek};
use byteorder::{ReadBytesExt, LittleEndian};

// Re-exports
pub use crate::error::*;

mod error;

/// A 3D vertex.
#[derive(Debug)]
pub struct Vertex {
    /// A 3D vector representing the vertex's world position.
    pub position: [f32; 3],
    /// A 3D vector representing the per vertex color.
    pub color: [u8; 3],
    /// A 2D vector representing the texture coordinates for the lightmap.
    pub tex_coords_lm: [f32; 2],
    /// A 2D vector representing the texture coordinates for the texture.
    pub tex_coords: [f32; 2],
}

/// A `Texture` represents a texture used by a mesh.
#[derive(Debug)]
pub struct Texture {
    /// The path to the texture file.
    pub path: String,
    /// The blend type of the texture.
    pub blend_type: u8,
}

/// A `Mesh` represents a mesh used in a [`RMesh`].
#[derive(Debug)]
pub struct Mesh {
    /// A vector containing the textures of the mesh.
    pub textures: Vec<Texture>,
    /// A vector containing the vertices of the mesh.
    pub vertices: Vec<Vertex>,
    /// A vector containing the triangles of the mesh.
    pub triangles: Vec<[u32; 3]>,
}

/// A `Collider` represents a collider in a [`RMesh`].
#[derive(Debug)]
pub struct Collider {
    /// A vector containing the vertices of the collider.
    pub vertices: Vec<[f32; 3]>,
    /// A vector containing the triangles of the collider.
    pub triangles: Vec<[u32; 3]>,
}

/// A `RMesh` represents a room mesh file that uses the rmesh extension found in scpcb.
#[derive(Debug)]
pub struct RMesh {
    /// A vector containing the meshes.
    pub meshes: Vec<Mesh>,
    /// A vector containing the colliders.
    pub colliders: Vec<Collider>,
}

impl RMesh {
    /// Parses a .rmesh file.
    /// 
    /// # Examples
    ///
    /// ```rust
    /// let bytes = std::fs::read("my_favorite_model.rmesh");
    ///
    /// let rmesh = rmesh::RMesh::read(&bytes)?;
    ///
    /// let positions: Vec<_> = rmesh.meshes[0].vertices.iter().map(|v| v.position).collect();
    /// let tex_coords: Vec<_> = rmesh.meshes[0].vertices.iter().map(|v| v.tex_coords).collect();
    /// ```
    pub fn read(data: &[u8]) -> Result<Self, RMeshError> {
        let mut cursor = Cursor::new(data);

        let tag = read_fixed_length_string(&mut cursor)?;

        if tag != "RoomMesh" && tag != "RoomMesh.HasTriggerBox" {
            return Err(RMeshError::InvalidHeader(tag).into())
        }

        let mut meshes = vec![];
        let mut colliders = vec![];

        let mesh_count = cursor.read_u32::<LittleEndian>()?;

        for _ in 0..mesh_count {
            let mut textures = vec![];
            let mut vertices = vec![];
            let mut triangles = vec![];

            for _ in 0..2 {
                let blend_type = cursor.read_u8()?;
                if blend_type != 0 {
                    let path = read_fixed_length_string(&mut cursor)?;
                    textures.push(Texture {
                        path,
                        blend_type,
                    });
                }
            }

            let vertex_count = cursor.read_u32::<LittleEndian>()?;

            for _ in 0..vertex_count {
                let mut position = [0.0; 3];
                cursor.read_f32_into::<LittleEndian>(&mut position)?;

                let mut tex_coords_lm = [0.0; 2];
                cursor.read_f32_into::<LittleEndian>(&mut tex_coords_lm)?;

                let mut tex_coords = [0.0; 2];
                cursor.read_f32_into::<LittleEndian>(&mut tex_coords)?;

                let mut color = [0; 3];
                cursor.read_exact(&mut color)?;

                vertices.push(Vertex {
                    position,
                    color,
                    tex_coords_lm,
                    tex_coords,
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

        let collider_count = cursor.read_u32::<LittleEndian>()?;

        for _ in 0..collider_count {
            let mut vertices = vec![];
            let mut triangles = vec![];

            let vertex_count = cursor.read_u32::<LittleEndian>()?;
            for _ in 0..vertex_count {
                let mut position = [0.0; 3];
                cursor.read_f32_into::<LittleEndian>(&mut position)?;
                
                vertices.push(position);
            }

            let poly_count = cursor.read_u32::<LittleEndian>()?;
            for _ in 0..poly_count {
                let mut tri = [0; 3];
                cursor.read_u32_into::<LittleEndian>(&mut tri)?;

                triangles.push(tri);
            }

            colliders.push(Collider {
                vertices,
                triangles,
            });
        }

        Ok(Self {
            meshes,
            colliders,
        })
    }
}

/// Reads a fixed-length string from a [`Cursor`].
/// 
/// # Examples
/// 
/// ```
/// use std::io::Cursor;
/// use rmesh::read_fixed_length_string;
/// 
/// let bytes = std::fs::read("my_favorite_model.rmesh");
/// 
/// let mut cursor = Cursor::new(&bytes);
/// 
/// let string = read_fixed_length_string(&mut cursor);
/// ```
pub fn read_fixed_length_string<T>(data: &mut T) -> Result<String, RMeshError>
where
    T: Read + Seek
{
    let len = data.read_u32::<LittleEndian>()?;
    let mut buf = vec![0; len as usize];
    data.read_exact(&mut buf)?;
    Ok(String::from_utf8(buf)?)
}
