use std::io::{Read, Seek};

use byteorder::{ReadBytesExt, LittleEndian};

use crate::{RMeshError, read_fixed_length_string, Vec3};

pub trait Entity: Sized {
    fn read<T>(data: &mut T) -> Result<Self, RMeshError> where T: Read + Seek;
}

#[derive(Debug)]
pub struct Screen {
    pub position: Vec3,
    pub name: String,
}

impl Entity for Screen {
    fn read<T>(data: &mut T) -> Result<Self, RMeshError>
    where
        T: Read + Seek
    {
        let mut position = [0.0; 3];
        data.read_f32_into::<LittleEndian>(&mut position)?;

        let name = read_fixed_length_string(data)?;

        Ok(Self {
            position,
            name,
        })
    }
}

#[derive(Debug)]
pub struct Waypoint {
    pub position: Vec3,
}

impl Entity for Waypoint {
    fn read<T>(data: &mut T) -> Result<Self, RMeshError>
    where
        T: Read + Seek
    {
        let mut position = [0.0; 3];
        data.read_f32_into::<LittleEndian>(&mut position)?;

        Ok(Self {
            position,
        })
    }
}

#[derive(Debug)]
pub struct Light {
    pub position: Vec3,
    pub range: f32,
    pub color: [u8; 3],
    pub intensity: f32,
}

impl Entity for Light {
    fn read<T>(data: &mut T) -> Result<Self, RMeshError>
    where
        T: Read + Seek
    {
        let mut position = [0.0; 3];
        data.read_f32_into::<LittleEndian>(&mut position)?;

        let range = data.read_f32::<LittleEndian>()?;
        let color_raw: Vec<_> = read_fixed_length_string(data)?.split(' ').map(|c| c.parse::<u8>().unwrap()).collect();
        let color = [
            color_raw[0],
            color_raw[1],
            color_raw[2],
        ];
        let intensity = data.read_f32::<LittleEndian>()?;

        Ok(Self {
            position,
            range,
            color,
            intensity,
        })
    }
}

#[derive(Debug)]
pub struct Spotlight {
    pub position: Vec3,
    pub range: f32,
    pub color: String,
    pub intensity: f32,
    pub angles: String,
    pub inner_cone_angle: f32,
    pub outer_cone_angle: f32,
}

impl Entity for Spotlight {
    fn read<T>(data: &mut T) -> Result<Self, RMeshError>
    where
        T: Read + Seek
    {
        let mut position = [0.0; 3];
        data.read_f32_into::<LittleEndian>(&mut position)?;

        let range = data.read_f32::<LittleEndian>()?;
        let color = read_fixed_length_string(data)?;
        let intensity = data.read_f32::<LittleEndian>()?;

        let angles = read_fixed_length_string(data)?;

        let inner_cone_angle = data.read_f32::<LittleEndian>()?;
        let outer_cone_angle = data.read_f32::<LittleEndian>()?;

        Ok(Self {
            position,
            range,
            color,
            intensity,
            angles,
            inner_cone_angle,
            outer_cone_angle,
        })
    }
}

#[derive(Debug)]
pub struct SoundEmitter {
    pub position: Vec3,
    pub idk0: u32,
    pub idk1: f32,
}

impl Entity for SoundEmitter {
    fn read<T>(data: &mut T) -> Result<Self, RMeshError>
    where
        T: Read + Seek
    {
        let mut position = [0.0; 3];
        data.read_f32_into::<LittleEndian>(&mut position)?;

        let idk0 = data.read_u32::<LittleEndian>()?;
        let idk1 = data.read_f32::<LittleEndian>()?;

        Ok(Self {
            position,
            idk0,
            idk1,
        })
    }
}

#[derive(Debug)]
pub struct PlayerStart {
    pub position: Vec3,
    pub angles: String,
}

impl Entity for PlayerStart {
    fn read<T>(data: &mut T) -> Result<Self, RMeshError>
    where
        T: Read + Seek
    {
        let mut position = [0.0; 3];
        data.read_f32_into::<LittleEndian>(&mut position)?;

        let angles = read_fixed_length_string(data)?;

        Ok(Self {
            position,
            angles,
        })
    }
}

#[derive(Debug)]
pub struct Model {
    pub name: String,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Entity for Model {
    fn read<T>(data: &mut T) -> Result<Self, RMeshError>
    where
        T: Read + Seek
    {
        let name = read_fixed_length_string(data)?;

        let mut position = [0.0; 3];
        data.read_f32_into::<LittleEndian>(&mut position)?;

        let mut rotation = [0.0; 3];
        data.read_f32_into::<LittleEndian>(&mut rotation)?;

        let mut scale = [0.0; 3];
        data.read_f32_into::<LittleEndian>(&mut scale)?;

        Ok(Self {
            name,
            position,
            rotation,
            scale,
        })
    }
}