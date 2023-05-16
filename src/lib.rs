use binrw::binrw;
use binrw::{BinRead, BinWrite, io::Cursor, BinReaderExt, BinWriterExt};

// Re-exports
pub use crate::entities::*;
pub use crate::error::RMeshError;
pub use crate::strings::*;

mod entities;
mod error;
mod strings;

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
    tag: FixedLengthString,
    
    #[bw(try_calc(u32::try_from(meshes.len())))]
    mesh_count: u32,
    
    #[br(count = mesh_count)]
    pub meshes: Vec<ComplexMesh>,
    
    #[bw(try_calc(u32::try_from(colliders.len())))]
    collider_count: u32,
    
    #[br(count = collider_count)]
    pub colliders: Vec<SimpleMesh>,
    
    #[bw(try_calc(u32::try_from(trigger_boxes.len())))]
    #[br(if(tag.values == b"RoomMesh.HasTriggerBox"))]
    trigger_boxes_count: u32,
    
    #[br(count = trigger_boxes_count, if(tag.values == b"RoomMesh.HasTriggerBox"))]
    pub trigger_boxes: Vec<TriggerBox>,
    
    #[bw(try_calc(u32::try_from(entities.len())))]
    entity_count: u32,
    
    #[br(count = entity_count)]
    pub entities: Vec<EntityData>,
}

#[binrw]
#[derive(Debug, Default)]
pub struct ComplexMesh {
    pub textures: [Texture; 2],
    
    #[bw(try_calc(u32::try_from(vertices.len())))]
    vertex_count: u32,
    
    #[br(count = vertex_count)]
    pub vertices: Vec<Vertex>,
    
    #[bw(try_calc(u32::try_from(triangles.len())))]
    triangle_count: u32,
    
    #[br(count = triangle_count)]
    pub triangles: Vec<[u32; 3]>,
}

#[derive(BinRead, BinWrite, Debug, Default)]
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
    None = 0,
    Visible = 1,
    Lightmap = 2,
    Transparent = 3,
}

#[derive(BinRead, BinWrite, Debug, Default)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [[f32; 2]; 2],
    pub color: [u8; 3],
}

#[derive(BinRead, BinWrite, Debug)]
pub struct SimpleMesh {
    pub vertex_count: u32,
    
    #[br(count = vertex_count)]
    pub vertices: Vec<[f32; 3]>,
    
    pub triangle_count: u32,
    
    #[br(count = triangle_count)]
    pub triangles: Vec<[u32; 3]>,
}

#[derive(BinRead, BinWrite, Debug)]
pub struct TriggerBox {
    pub mesh_count: u32,
    
    #[br(count = mesh_count)]
    pub meshes: Vec<SimpleMesh>,

    pub name: FixedLengthString,
}

#[derive(BinRead, BinWrite, Debug)]
pub struct EntityData {
    entity_name_size: u32,
    pub entity_type: Option<EntityType>,
}

#[derive(BinRead, BinWrite, Debug)]
pub enum EntityType {
    #[br(magic = b"screen")] Screen(EntityScreen),
    #[br(magic = b"waypoint")] WayPoint(EntityWaypoint),
    #[br(magic = b"light")] Light(EntityLight),
    #[br(magic = b"spotlight")] SpotLight(EntitySpotlight),
    #[br(magic = b"soundemitter")] SoundEmitter(EntitySoundEmitter),
    #[br(magic = b"playerstart")] PlayerStart(EntityPlayerStart),
    #[br(magic = b"model")] Model(EntityModel),
}

/// Reads a .rmesh file.
/// 
/// # Examples
/// 
/// ```rust
/// let bytes = std::fs::read("GFX/map/lockroom_opt.rmesh").unwrap();
/// let rmesh = read_rmeshad(&bytes).unwrap();
/// assert_eq!(rmesh.colliders.len(), 0);
/// assert_eq!(rmesh.entities.len(), 13);
/// ```
pub fn read_rmesh(bytes: &[u8]) -> Result<Header, RMeshError> {
    let mut cursor = Cursor::new(bytes);
    let header: Header = cursor.read_le()?;
    Ok(header)
}

/// Writes a .rmesh file.
/// 
/// # Examples
/// Creating a 2D triangle.
///
/// ```rust
/// let header = Header {
///     meshes: vec![
///         ComplexMesh {
///             vertices: vec![
///                 Vertex { position: [0.0, 0.0, 0.0], ..Default::default() },
///                 Vertex { position: [0.5, 1.0, 0.0], ..Default::default() },
///                 Vertex { position: [1.0, 0.0, 0.0], ..Default::default() },
///             ],
///             triangles: vec![
///                 [0, 1, 2]
///             ],
///             ..Default::default()
///         }
///     ],
///     ..Default::default()
/// };
/// let rmesh = write_rmesh(&header)?;
/// let mut file = File::create("assets/cube.rmesh").unwrap();
/// file.write_all(&rmesh).unwrap();
/// ```
pub fn write_rmesh(header: &Header) -> Result<Vec<u8>, RMeshError> {
    let mut bytes = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);

    cursor.write_le(header)?;

    Ok(bytes)
}
