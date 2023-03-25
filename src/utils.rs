use std::io::{Read, Seek};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::RMeshError;

/// A `Vec3` contains three floats and is used for 3D.
pub type Vec3 = [f32; 3];

/// A `Vec2` contains three floats and is used for 2D.
pub type Vec2 = [f32; 2];

/// A `UVec3` contains unsigned-integer floats and is used for 3D.
pub type UVec3 = [u32; 3];

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
