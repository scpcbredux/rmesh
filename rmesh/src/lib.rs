use std::io::Cursor;

use binrw::binrw;
use binrw::prelude::*;

// Re-exports
pub use crate::entities::*;
pub use crate::error::RMeshError;
pub use crate::strings::*;

mod entities;
mod error;
mod strings;

pub const ROOM_SCALE: f32 = 8. / 2048.;

pub fn header_tag(trigger_box_count: usize) -> Result<FixedLengthString, RMeshError> {
    if trigger_box_count > 0 {
        Ok("RoomMesh.HasTriggerBox".into())
    } else {
        Ok("RoomMesh".into())
    }
}

#[binrw]
#[derive(Debug, Default)]
pub struct Header {
    #[bw(try_calc(header_tag(trigger_boxes.len())))]
    pub kind: FixedLengthString,

    #[bw(try_calc(u32::try_from(meshes.len())))]
    mesh_count: u32,

    #[br(count = mesh_count)]
    pub meshes: Vec<ComplexMesh>,

    #[bw(try_calc(u32::try_from(colliders.len())))]
    #[br(temp)]
    collider_count: u32,

    #[br(count = collider_count)]
    pub colliders: Vec<SimpleMesh>,

    #[bw(try_calc(u32::try_from(trigger_boxes.len())))]
    #[br(temp, if(kind.values == b"RoomMesh.HasTriggerBox"))]
    trigger_boxes_count: u32,

    #[br(count = trigger_boxes_count, if(kind.values == b"RoomMesh.HasTriggerBox"))]
    pub trigger_boxes: Vec<TriggerBox>,

    #[bw(try_calc(u32::try_from(entities.len())))]
    #[br(temp)]
    entity_count: u32,

    #[br(count = entity_count)]
    pub entities: Vec<EntityData>,
}

#[binrw]
#[derive(Debug, Default)]
pub struct ComplexMesh {
    pub textures: [Texture; 2],

    #[bw(try_calc(u32::try_from(vertices.len())))]
    #[br(temp)]
    vertex_count: u32,

    #[br(count = vertex_count)]
    pub vertices: Vec<Vertex>,

    #[bw(try_calc(u32::try_from(triangles.len())))]
    #[br(temp)]
    triangle_count: u32,

    #[br(count = triangle_count)]
    pub triangles: Vec<[u32; 3]>,
}

#[binrw]
#[derive(Debug, Default)]
pub struct Texture {
    pub blend_type: TextureBlendType,

    #[br(if(blend_type != TextureBlendType::None))]
    pub path: Option<FixedLengthString>,
}

#[binrw]
#[brw(repr(u8))]
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextureBlendType {
    #[default]
    None,
    Visible,
    Lightmap,
    Transparent,
}

#[binrw]
#[derive(Debug, Default)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [[f32; 2]; 2],
    pub color: [u8; 3],
}

#[binrw]
#[derive(Debug)]
pub struct SimpleMesh {
    pub vertex_count: u32,

    #[br(count = vertex_count)]
    pub vertices: Vec<[f32; 3]>,

    pub triangle_count: u32,

    #[br(count = triangle_count)]
    pub triangles: Vec<[u32; 3]>,
}

#[binrw]
#[derive(Debug)]
pub struct TriggerBox {
    #[bw(try_calc(u32::try_from(meshes.len())))]
    #[br(temp)]
    pub mesh_count: u32,

    #[br(count = mesh_count)]
    pub meshes: Vec<SimpleMesh>,

    pub name: FixedLengthString,
}

impl CalcBoundBox for SimpleMesh {
    fn bounding_box(&self) -> Bounds {
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut min_z = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        let mut max_z = f32::NEG_INFINITY;

        for vertex in &self.vertices {
            let [x, y, z] = *vertex;

            // Update min values
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            min_z = min_z.min(z);

            // Update max values
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            max_z = max_z.max(z);
        }

        let min_point = [min_x, min_y, min_z];
        let max_point = [max_x, max_y, max_z];
        Bounds::new(min_point, max_point)
    }
}

impl CalcBoundBox for ComplexMesh {
    fn bounding_box(&self) -> Bounds {
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut min_z = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        let mut max_z = f32::NEG_INFINITY;

        for vertex in &self.vertices {
            let [x, y, z] = vertex.position;

            // Update min values
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            min_z = min_z.min(z);

            // Update max values
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            max_z = max_z.max(z);
        }

        let min_point = [min_x, min_y, min_z];
        let max_point = [max_x, max_y, max_z];
        Bounds::new(min_point, max_point)
    }
}

pub trait CalcBoundBox {
    /// Used for aabb calc
    fn bounding_box(&self) -> Bounds;
}

pub struct Bounds {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl Bounds {
    pub fn new(min: [f32; 3], max: [f32; 3]) -> Self {
        Self { min, max }
    }
}

#[binrw]
#[derive(Debug)]
pub struct EntityData {
    entity_name_size: u32,
    pub entity_type: Option<EntityType>,
}

#[binrw]
#[derive(Debug)]
pub enum EntityType {
    #[br(magic = b"screen")]
    Screen(EntityScreen),
    #[br(magic = b"waypoint")]
    WayPoint(EntityWaypoint),
    #[br(magic = b"light")]
    Light(EntityLight),
    #[br(magic = b"spotlight")]
    SpotLight(EntitySpotlight),
    #[br(magic = b"soundemitter")]
    SoundEmitter(EntitySoundEmitter),
    #[br(magic = b"playerstart")]
    PlayerStart(EntityPlayerStart),
    #[br(magic = b"model")]
    Model(EntityModel),
}

/// Reads a .rmesh file.
pub fn read_rmesh(bytes: &[u8]) -> Result<Header, RMeshError> {
    let mut cursor = Cursor::new(bytes);
    let header: Header = cursor.read_le()?;
    Ok(header)
}

/// Writes a .rmesh file.
pub fn write_rmesh(header: &Header) -> Result<Vec<u8>, RMeshError> {
    let mut bytes = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);

    cursor.write_le(header)?;

    Ok(bytes)
}
