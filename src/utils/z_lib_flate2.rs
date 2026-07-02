use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use std::io::{Read, Result, Write};

// It's just a thin wrapper around 'flate2'.

pub fn inflate(data: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(data);
    let mut uncompressed = Vec::new();
    decoder
        .read_to_end(&mut uncompressed)
        .map(|_sz| uncompressed)
}

pub fn deflate(data: &[u8]) -> Result<Vec<u8>> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::best());
    e.write_all(data)?;
    e.finish()
}
