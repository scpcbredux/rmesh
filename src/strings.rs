use std::fmt;

use binrw::{BinRead, BinWrite};

#[derive(BinRead, BinWrite, Clone, Eq, PartialEq, Default)]
pub struct FixedLengthString {
    pub len: u32,
    #[br(count = len)]
    pub values: Vec<u8>,
}

impl fmt::Debug for FixedLengthString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FixedLengthString(\"")?;
        write!(f, "{}", String::from_utf8(self.values.clone()).unwrap())?;
        write!(f, "\")")
    }
}

impl From<&str> for FixedLengthString {
    fn from(s: &str) -> Self {
        let values = s.as_bytes().to_vec();
        Self {
            len: values.len() as u32,
            values,
        }
    }
}

impl From<String> for FixedLengthString {
    fn from(s: String) -> Self {
        let values = s.into_bytes();
        Self {
            len: values.len() as u32,
            values,
        }
    }
}

impl From<FixedLengthString> for String {
    fn from(value: FixedLengthString) -> Self {
        String::from_utf8(value.values.clone()).unwrap()
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct ThreeTypeString(pub Vec<u8>);

impl BinRead for ThreeTypeString {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let len = <u32>::read_options(reader, endian, ())?;

        let mut values = vec![];

        for _ in 0..len {
            let val = <u8>::read_options(reader, endian, ())?;
            values.push(val);
        }

        let string = String::from_utf8(values).unwrap();
        let stuff: Vec<_> = string
            .split(' ')
            .map(|c| c.parse::<u8>().unwrap())
            .collect();

        Ok(Self(stuff))
    }
}

impl BinWrite for ThreeTypeString {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        let string = self.0.iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let bytes = string.into_bytes();
        let len = bytes.len() as u32;

        len.write_options(writer, endian, ())?;
        writer.write_all(&bytes[..])?;

        Ok(())
    }
}

impl From<Vec<u8>> for ThreeTypeString {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<[u8; 3]> for ThreeTypeString {
    fn from(value: [u8; 3]) -> Self {
        Self(value.to_vec())
    }
}
