use std::io::{Read, Result};

pub mod image;

#[cfg(feature = "flate2")]
pub mod z_lib_flate2;

#[cfg(feature = "libflate")]
pub mod z_lib_libflate;

#[cfg(feature = "flate2")]
pub use self::z_lib_flate2 as z_lib;
#[cfg(all(feature = "libflate", not(feature = "flate2")))]
pub use self::z_lib_libflate as z_lib;

pub fn load_raw<R: Read>(mut reader: R) -> Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(8192);
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
