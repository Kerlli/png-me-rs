mod chunk_type;
pub mod image_header;
pub mod palette;
mod gamma;
mod chromaticities;
mod srgb;
mod icc_profile;
mod textual;
pub mod transparency;

pub use self::chunk_type::ChunkType;

use self::chromaticities::ChunkChromaticities;
use self::icc_profile::ChunkICCProfile;
use self::image_header::{ChunkImageHeader, IMAGE_HEADER_CHUNK_DATA_LEN};
use self::gamma::ChunkGamma;
use self::palette::ChunkPalette;
use self::srgb::ChunkSRGB;
use self::textual::*;
use self::transparency::ChunkTransparency;

use std::fmt;
use std::fmt::Display;
use crc::{Crc,CRC_32_ISO_HDLC};
use super::{ColorType, PngError};

const CRC_CKSUM: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

pub const CHUNK_LENGTH_BYTE_LEN: usize = 4;
pub const CHUNK_TYPE_BYTE_LEN: usize = 4;
pub const CHUNK_CRC_BYTE_LEN: usize = 4;

pub enum ChunkData {
  ImageHeader(ChunkImageHeader),
  Palette(ChunkPalette),
  ImageData(Vec<u8>),
  ImageEnd,
  Transparency(ChunkTransparency),
  Gamma(ChunkGamma),
  Chromaticities(ChunkChromaticities),
  SRgb(ChunkSRGB),
  ICCProfile(ChunkICCProfile),
  Textual(ChunkTextual),
  TextualCompressed(ChunkTextualCompressed),
  TextualInternational(ChunkTextualInternational),
  Other(Vec<u8>),
}

impl Display for ChunkData {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    match self {
      ChunkData::ImageHeader(header) => write!(f, "{}", header.to_string()),
      ChunkData::Palette(palette) => write!(f, "{}", palette.to_string()),
      ChunkData::ImageData(data) => write!(f, "[u8](len: {})", data.len()),
      ChunkData::ImageEnd => write!(f, "[Empty]"),
      ChunkData::Gamma(gamma) => write!(f, "Gamma: {}", gamma.to_string()),
      ChunkData::Chromaticities(ch) => write!(f, "Chromaticties: {}", ch.to_string()),
      ChunkData::SRgb(srgb) => write!(f, "sRGB rendering intent: {}", srgb.to_string()),
      ChunkData::ICCProfile(profile) => write!(f, "ICC profile: {}", profile.to_string()),
      ChunkData::Textual(t) => write!(f, "Textual: {}", t.to_string()),
      ChunkData::TextualCompressed(tc) => write!(f, "Textual compressed: {}", tc.to_string()),
      ChunkData::TextualInternational(ti) => write!(f, "Textual international: {}", ti.to_string()),
      ChunkData::Other(data) => {
        let s = String::from_utf8(data.to_vec());
        match s {
          Ok(s) => write!(f, "{:?}", s),
          Err(_) => write!(f, "[Content cannot be decoded]")
        }
      },
      _ => write!(f, "Not impl yet")
    }
  }
}

pub(super) trait ChunkRawBytes {
  fn as_bytes(&self) -> Vec<u8>;
}

impl ChunkData {
  pub fn as_bytes(&self) -> Vec<u8> {
    match self {
      ChunkData::ImageHeader(header) => header.as_bytes(),
      ChunkData::Palette(palette) => palette.as_bytes(),
      ChunkData::ImageData(data) => data.to_vec(),
      ChunkData::ImageEnd => vec![],
      ChunkData::Transparency(trans) => trans.as_bytes(),
      ChunkData::Gamma(gamma) => gamma.as_bytes(),
      ChunkData::Chromaticities(ch) => ch.as_bytes(),
      ChunkData::SRgb(srgb) => srgb.as_bytes(),
      ChunkData::ICCProfile(icc) => icc.as_bytes(),
      ChunkData::Textual(t) => t.as_bytes(),
      ChunkData::TextualCompressed(tc) => tc.as_bytes(),
      ChunkData::TextualInternational(ti) => ti.as_bytes(),
      ChunkData::Other(data) => data.to_vec(),
    }
  }
}

#[allow(dead_code)]
pub struct Chunk {
  length: u32,
  chunk_type: ChunkType,
  data: ChunkData,
  crc: u32,
}

fn map_chunk_data(chunk_type: &ChunkType, data: Vec<u8>) -> ChunkData {
  match chunk_type.to_string().as_str() {
    "IHDR" => {
      let mut bytes: [u8; IMAGE_HEADER_CHUNK_DATA_LEN] = [0; 13];
      for (i, b) in data.iter().enumerate() {
        bytes[i] = *b;
      } 
      let header = ChunkImageHeader::try_from(bytes).unwrap();
      ChunkData::ImageHeader(header)
    },
    "IDAT" => ChunkData::ImageData(data),
    "iCCP" => {
      let icc_profile = ChunkICCProfile::try_from(&data).unwrap();
      ChunkData::ICCProfile(icc_profile)
    },
    "IEND" => ChunkData::ImageEnd,
    "tEXt" => {
      let text = ChunkTextual::try_from(&data[..]).unwrap();
      ChunkData::Textual(text)
    },
    // todo: chunk compressed
    // todo: chunk international
    _ => ChunkData::Other(data),
  }
}

impl TryFrom<&[u8]> for Chunk {
  type Error = PngError;

  fn try_from(v: &[u8]) -> Result<Self, <Self as TryFrom<&[u8]>>::Error> {
    let len = v.len();

    let mut length_bytes: [u8; CHUNK_LENGTH_BYTE_LEN] = [0; CHUNK_LENGTH_BYTE_LEN];

    let iter_length = v.iter().take(CHUNK_LENGTH_BYTE_LEN);

    for (i, b) in iter_length.enumerate() {
      length_bytes[i] = *b;
    }

    let length: u32 = u32::from_be_bytes(length_bytes);

    let mut chunk_bytes: [u8; CHUNK_TYPE_BYTE_LEN] = [0; CHUNK_TYPE_BYTE_LEN];

    for (i, b) in v.iter().skip(CHUNK_LENGTH_BYTE_LEN).take(CHUNK_TYPE_BYTE_LEN).enumerate() {
      chunk_bytes[i] = *b;
    }

    let chunk_type: ChunkType = ChunkType::try_from(chunk_bytes)?;

    let bytes_len_before_data = CHUNK_LENGTH_BYTE_LEN + CHUNK_TYPE_BYTE_LEN;
    let except_data_len = bytes_len_before_data + CHUNK_CRC_BYTE_LEN;
    let data_len = len - except_data_len;

    let mut data: Vec<u8> = Vec::with_capacity(data_len);

    for b in v.iter().skip(bytes_len_before_data).take(data_len) {
      data.push(*b);
    }

    let mut crc_bytes: [u8; CHUNK_CRC_BYTE_LEN] = [0; CHUNK_CRC_BYTE_LEN];

    for (i, b) in v.iter().skip(len - CHUNK_CRC_BYTE_LEN).take(CHUNK_CRC_BYTE_LEN).enumerate() {
      crc_bytes[i] = *b;
    }

    let crc_checksum = CRC_CKSUM.checksum(
      chunk_type.bytes().iter()
        .chain(data.iter())
        .map(|v| *v)
        .collect::<Vec<u8>>()
        .as_slice()
    );

    let crc = u32::from_be_bytes(crc_bytes);

    if crc != crc_checksum {
      return Err(PngError::ChunkCrcMismatch)
    }

    let chunk_data = map_chunk_data(&chunk_type, data);

    Ok(Self {
      length,
      chunk_type,
      data: chunk_data,
      crc,
    })
  }
}

impl Display for Chunk {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    write!(f, "Chunk [{}]\nLength: {}\nData: {}\nCrc: {}\n", self.chunk_type().to_string(), self.length, self.data.to_string(), self.crc)
  }
}

impl Chunk {
  pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
    let length = data.len() as u32;

    let crc = CRC_CKSUM.checksum(
      chunk_type.bytes().iter()
        .chain(data.iter())
        .map(|v| *v)
        .collect::<Vec<u8>>()
        .as_slice()
    );

    Self {
      length,
      chunk_type,
      data: map_chunk_data(&chunk_type, data),
      crc,
    }
  }

  pub fn length(&self) -> u32 {
    self.length
  }

  pub fn chunk_type(&self) -> &ChunkType {
    &self.chunk_type
  }

  pub fn chunk_data(&self) -> &ChunkData {
    &self.data
  }

  pub fn data(&self) -> Vec<u8> {
    self.data.as_bytes()
  }

  pub fn crc(&self) -> u32 {
    self.crc
  }

  pub fn set_data(&mut self, data: &[u8]) {
    let new_len = data.len();
    let chunk_type = self.chunk_type;
    let new_data = map_chunk_data(&chunk_type, data.to_vec());

    let new_crc = CRC_CKSUM.checksum(
      chunk_type.bytes().iter()
        .chain(data.iter())
        .map(|v| *v)
        .collect::<Vec<u8>>()
        .as_slice()
    );

    self.length = new_len as u32;
    self.data = new_data;
    self.crc = new_crc;
  }

  pub fn data_as_string(&self) -> Result<String, std::string::FromUtf8Error> {
    let s = String::from_utf8(self.data.as_bytes().iter().map(|v| *v).collect::<Vec<u8>>())?;

    Ok(s)
  }

  pub fn as_bytes(&self) -> Vec<u8> {
    self.length.to_be_bytes().iter()
      .chain(self.chunk_type.bytes().iter())
      .chain(self.data.as_bytes().iter())
      .chain(self.crc.to_be_bytes().iter())
      .map(|&v| v)
      .collect()
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}



