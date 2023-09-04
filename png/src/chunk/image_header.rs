use std::fmt;
use std::fmt::Display;
use crate::ChunkDataDecodeable;
use crate::error::PngError;

use super::ColorType;

pub const IMAGE_HEADER_CHUNK_DATA_LEN: usize = 13;

/// IHDR (Image header) chunk
/// structure: <br/>
/// Width: 4 bytes <br/>
/// Height: 4 bytes <br/>
/// Bit depth: 1 byte <br/>
/// Color type: 1 byte <br/>
/// Compression method: 1 byte <br/>
/// Filter method: 1 byte <br/>
/// Interlace method: 1 byte <br/>
#[derive(PartialEq)]
pub struct ChunkImageHeader {
  width: u32,
  height: u32,
  bit_depth: u8,
  color_type: ColorType,
  compression_method: u8,
  filter_method: u8,
  interlace_method: u8,
}

impl Display for ChunkImageHeader {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    write!(
      f,
      "Width: {}, Height: {}\nBit depth: {}, Color type: {}",
      self.width,
      self.height,
      self.bit_depth,
      self.color_type,
    )
  }
}

impl TryFrom<[u8; IMAGE_HEADER_CHUNK_DATA_LEN]> for ChunkImageHeader {
  type Error = PngError;

  fn try_from(bytes: [u8; IMAGE_HEADER_CHUNK_DATA_LEN]) -> Result<Self, Self::Error> {
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    // width
    if let [w1, w2, w3, w4] = &bytes[0..4] {
      width = u32::from_be_bytes([*w1, *w2, *w3, *w4]);
    }
    // height
    if let [h1, h2, h3, h4] = &bytes[4..8] {
      height = u32::from_be_bytes([*h1, *h2, *h3, *h4]);
    }
    // bit depth
    let bit_depth = &bytes[8];
    // color type
    let color_type_raw = &bytes[9];

    let color_type = ColorType::try_from(*color_type_raw)?;

    let group = (*color_type_raw, *bit_depth);

    let valid_bit_depth = match group {
      (0, 1 | 2 | 4 | 8 | 16) => true,
      (2, 8 | 16) => true,
      (3, 1 | 2 | 4 | 8) => true,
      (4, 8 | 16) => true,
      (6, 8 | 16) => true,
      _ => false,
    };

    if !valid_bit_depth {
      return Err(PngError::InvalidBitDepth)
    }

    // compression method
    // at present, only 0(deflate/inflate) is accepted
    let compression_method = &bytes[10];

    if *compression_method != 0 {
      return Err(PngError::InvalidCompressionMehtod)
    }

    // filter method
    // at present, only 0 is accepted
    let filter_method = &bytes[11];

    if *filter_method != 0 {
      return Err(PngError::InvalidFilterMethod)
    }

    // interlace method
    // 0 and 1 is accepted
    let interlace_method = &bytes[12];

    if !(*interlace_method == 0 || *interlace_method == 1) {
      return Err(PngError::InvalidInterlaceMethod)
    }

    Ok(Self {
      width,
      height,
      bit_depth: *bit_depth,
      color_type,
      compression_method: *compression_method,
      filter_method: *filter_method,
      interlace_method: *interlace_method,
    })
  }
}

impl ChunkDataDecodeable for ChunkImageHeader {
  fn as_bytes(&self) -> Vec<u8> {
    self.width.to_be_bytes().iter()
      .chain(self.height.to_be_bytes().iter())
      .chain(Some(self.bit_depth).iter())
      .chain(Some(self.color_type.clone().into()).iter())
      .chain(Some(self.compression_method).iter())
      .chain(Some(self.filter_method.clone().into()).iter())
      .chain(Some(self.interlace_method).iter())
      .map(|&v| v)
      .collect()
  }
}

impl ChunkImageHeader {
  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn bit_depth(&self) -> u8 {
    self.bit_depth
  }

  pub fn color_type(&self) -> ColorType {
    self.color_type.clone()
  }

  pub fn color_channels(&self) -> u8 {
    self.color_type.channels()
  }

  pub fn compression_method(&self) -> u8 {
    self.compression_method
  }

  pub fn filter_method(&self) -> u8 {
    self.filter_method.clone().into()
  }

  pub fn interface_method(&self) -> u8 {
    self.interlace_method
  }
}

