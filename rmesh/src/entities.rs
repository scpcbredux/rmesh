use binrw::{BinRead, BinWrite};

use crate::strings::{FixedLengthString, ThreeTypeString};

#[derive(BinRead, BinWrite, Debug)]
pub struct EntityScreen {
    pub position: [f32; 3],
    pub name: FixedLengthString,
}

#[derive(BinRead, BinWrite, Debug)]
pub struct EntityWaypoint {
    pub position: [f32; 3],
}

#[derive(BinRead, BinWrite, Debug)]
pub struct EntityLight {
    pub position: [f32; 3],
    pub range: f32,
    pub color: ThreeTypeString,
    pub intensity: f32,
}

#[derive(BinRead, BinWrite, Debug)]
pub struct EntitySpotlight {
    pub position: [f32; 3],
    pub range: f32,
    pub color: ThreeTypeString,
    pub intensity: f32,
    pub angles: ThreeTypeString,
    pub inner_cone_angle: f32,
    pub outer_cone_angle: f32,
}

#[derive(BinRead, BinWrite, Debug)]
pub struct EntitySoundEmitter {
    pub position: [f32; 3],
    pub idk0: u32,
    pub idk1: f32,
}

#[derive(BinRead, BinWrite, Debug)]
pub struct EntityPlayerStart {
    pub position: [f32; 3],
    pub angles: ThreeTypeString,
}

#[derive(BinRead, BinWrite, Debug)]
pub struct EntityModel {
    pub name: FixedLengthString,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}
