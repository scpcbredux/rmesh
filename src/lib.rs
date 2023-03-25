#![warn(missing_docs)]

//! # RMesh
//! 
//! `rmesh` is a library for reading and parsing room meshes that use the rmesh extension found in scpcb.

use std::io::{Cursor, Read, Seek};
use std::vec;
use byteorder::{ReadBytesExt, LittleEndian};

// Re-exports
pub use crate::error::*;
pub use crate::utils::*;
pub use crate::entities::*;

mod error;
mod utils;
mod entities;

/// A 3D vertex.
#[derive(Debug)]
pub struct Vertex {
    /// A 3D vector representing the vertex's world position.
    pub position: Vec3,
    /// A 3D vector representing the per vertex color.
    pub color: [u8; 3],
    /// A 2D vector representing the texture coordinates for the lightmap.
    pub tex_coords_lm: Vec2,
    /// A 2D vector representing the texture coordinates for the texture.
    pub tex_coords: Vec2,
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
    pub triangles: Vec<UVec3>,
}

/// A `SimpleMesh` represents a collider/trigger box mesh in a [`RMesh`].
#[derive(Debug)]
pub struct SimpleMesh {
    /// A vector containing the vertices.
    pub vertices: Vec<Vec3>,
    /// A vector containing the triangles.
    pub triangles: Vec<UVec3>,
}

/// A `TriggerBox` represents a trigger box in a [`RMesh`].
#[derive(Debug)]
pub struct TriggerBox {
    /// The name of the trigger box.
    pub name: String,
    /// A vector containing the meshes of the trigger box.
    pub meshes: Vec<SimpleMesh>,
}

/// The `Entities` represents the entities in a [`RMesh`].
#[derive(Debug)]
pub struct Entities {
    pub screens: Vec<Screen>,
    pub waypoints: Vec<Waypoint>,
    pub lights: Vec<Light>,
    pub spotlights: Vec<Spotlight>,
    pub sound_emitters: Vec<SoundEmitter>,
    pub player_starts: Vec<PlayerStart>,
    pub models: Vec<Model>,
}

/// A `RMesh` represents a room mesh file that uses the rmesh extension found in scpcb.
#[derive(Debug)]
pub struct RMesh {
    /// A vector containing the meshes.
    pub meshes: Vec<Mesh>,
    /// A vector containing the colliders.
    pub colliders: Vec<SimpleMesh>,
    /// A vector containing the trigger boxes.
    pub trigger_boxes: Vec<TriggerBox>,
    /// The entities (such as; player start, spotlights, etc).
    pub entities: Entities,
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

        let mut colliders = vec![];
        let collider_count = cursor.read_u32::<LittleEndian>()?;

        // TODO: Colliders weren't tested
        for _ in 0..collider_count {
            colliders.push(Self::read_simple_mesh(&mut cursor)?);
        }

        // TODO: Trigger boxes weren't tested
        let trigger_boxes = if tag == "RoomMesh.HasTriggerBox" {
            Self::read_trigger_boxes(&mut cursor)?
        } else {
            vec![]
        };

        let entities = Self::read_entities(&mut cursor)?;

        Ok(Self {
            meshes,
            colliders,
            trigger_boxes,
            entities,
        })
    }

    fn read_trigger_boxes<T>(data: &mut T) -> Result<Vec<TriggerBox>, RMeshError>
    where
        T: Read + Seek
    {
        let mut trigger_boxes = vec![];

        let trigger_box_amount = data.read_u32::<LittleEndian>()?;
        for _ in 0..trigger_box_amount - 1 {
            let mut meshes = vec![];

            let mesh_count = data.read_u32::<LittleEndian>()?;
            for _ in 0..mesh_count {
                meshes.push(Self::read_simple_mesh(data)?);
            }

            trigger_boxes.push(TriggerBox {
                name: read_fixed_length_string(data)?,
                meshes,
            });
        }

        Ok(trigger_boxes)
    }

    fn read_simple_mesh<T>(data: &mut T) -> Result<SimpleMesh, RMeshError>
    where
        T: Read + Seek
    {
        let mut vertices = vec![];
        let mut triangles = vec![];

        let vertex_count = data.read_u32::<LittleEndian>()?;
        for _ in 0..vertex_count {
            let mut position = [0.0; 3];
            data.read_f32_into::<LittleEndian>(&mut position)?;
            
            vertices.push(position);
        }

        let poly_count = data.read_u32::<LittleEndian>()?;
        for _ in 0..poly_count {
            let mut tri = [0; 3];
            data.read_u32_into::<LittleEndian>(&mut tri)?;

            triangles.push(tri);
        }

        Ok(SimpleMesh {
            vertices,
            triangles,
        })
    }

    fn read_entities<T>(data: &mut T) -> Result<Entities, RMeshError>
    where
        T: Read + Seek,
    {
        let mut screens = vec![];
        let mut waypoints = vec![];
        let mut lights = vec![];
        let mut spotlights = vec![];
        let mut sound_emitters = vec![];
        let mut player_starts = vec![];
        let mut models = vec![];

        let entities_count = data.read_u32::<LittleEndian>()?;

        for _ in 0..entities_count {
            let entity_name = read_fixed_length_string(data)?;

            match entity_name.as_str() {
                "screen" => {
                    let screen = Screen::read(data)?;
                    screens.push(screen);
                }
                "waypoint" => {
                    let waypoint = Waypoint::read(data)?;
                    waypoints.push(waypoint);
                }
                "light" => {
                    let light = Light::read(data)?;
                    lights.push(light);
                }
                "spotlight" => {
                    let spotlight = Spotlight::read(data)?;
                    spotlights.push(spotlight);
                }
                "soundemitter" => {
                    let sound_emitter = SoundEmitter::read(data)?;
                    sound_emitters.push(sound_emitter);
                }
                "playerstart" => {
                    let player_start = PlayerStart::read(data)?;
                    player_starts.push(player_start);
                }
                "model" => {
                    let model = Model::read(data)?;
                    models.push(model);
                }
                _ => {}
            }
        }

        Ok(Entities {
            screens,
            waypoints,
            lights,
            spotlights,
            sound_emitters,
            player_starts,
            models,
        })
    }
}
