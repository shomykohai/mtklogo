use super::StartExt;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Error as IOError, ErrorKind, Read, Result, Write};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// An MTK image header.
pub struct MtkHeader {
    pub size: u32,
    pub mtk_type: MtkType,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MtkType {
    Recovery,
    Rootfs,
    Kernel,
    Logo,
}

impl MtkType {
    /// Tests whether the specified "magic bytes" correspond to some possible mtk image type.
    fn from_bytes(bytes: &[u8]) -> Option<MtkType> {
        const LABELS: [(MtkType, &[u8]); 4] = [
            (MtkType::Recovery, b"RECOVERY"),
            (MtkType::Rootfs, b"ROOTFS"),
            (MtkType::Kernel, b"KERNEL"),
            (MtkType::Logo, b"LOGO"),
        ];
        for (mtk_type, label) in LABELS {
            if bytes.starts_with_ascii_ignore_case(label) {
                return Some(mtk_type);
            }
        }
        None
    }
}

impl MtkHeader {
    pub const SIZE: usize = 512;
    pub const FILL: u8 = 0xFF;
    pub const MAGIC: u32 = 0x88168858;

    /// Reads an header.
    pub fn read<R: Read>(reader: &mut R) -> Result<MtkHeader> {
        let magic = reader.read_u32::<BigEndian>()?;
        // Assert is magic flag.
        if magic != Self::MAGIC {
            return Err(IOError::new(ErrorKind::InvalidData, "missing magic number"));
        }
        let size = reader.read_u32::<LittleEndian>()?;
        let mut typ = [0u8; 32];
        reader.read_exact(&mut typ)?;
        let mtk_type = MtkType::from_bytes(&typ)
            .ok_or_else(|| IOError::new(ErrorKind::InvalidData, "Missing MTK Header type."))?;

        let mut remainder = [0u8; 472];
        reader.read_exact(&mut remainder)?;
        Ok(MtkHeader { size, mtk_type })
    }

    /// Writes this header to the specified writer.
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u32::<BigEndian>(Self::MAGIC)?;
        writer.write_u32::<LittleEndian>(self.size)?;
        let mut imagetype = [0u8; 32];
        let label = match self.mtk_type {
            MtkType::Logo => "LOGO",
            MtkType::Recovery => "RECOVERY",
            MtkType::Kernel => "KERNEL",
            MtkType::Rootfs => "ROOTFS",
        };
        imagetype[..label.len()].copy_from_slice(label.as_bytes());
        writer.write_all(&imagetype)?;
        let remainder = [Self::FILL; 472];
        writer.write_all(&remainder)?;
        Ok(())
    }
}
