use std::fmt;
use std::fmt::Display;
use crate::ChunkRawBytes;

pub struct ChunkChromaticities {
  white_point_x: u32,
  white_point_y: u32,
  red_x: u32,
  red_y: u32,
  green_x: u32,
  green_y: u32,
  blue_x: u32,
  blue_y: u32,
}

impl Display for ChunkChromaticities {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    write!(
      f,
      "cHRM:\nWhite Point x: {}\n, White Point y: {}\n, Red x: {}\n, Red y: {}\n, Green x: {}\n, Green y: {}\n, Blue x: {}\n, Blue y: {}",
      self.white_point_x,
      self.white_point_y,
      self.red_x,
      self.red_y,
      self.green_x,
      self.green_y,
      self.blue_x,
      self.blue_y,
    )
  }
}

impl ChunkRawBytes for ChunkChromaticities {
  fn as_bytes(&self) -> Vec<u8> {
    self.white_point_x.to_be_bytes().iter()
      .chain(self.white_point_y.to_be_bytes().iter())
      .chain(self.red_x.to_be_bytes().iter())
      .chain(self.red_y.to_be_bytes().iter())
      .chain(self.green_x.to_be_bytes().iter())
      .chain(self.green_y.to_be_bytes().iter())
      .chain(self.blue_x.to_be_bytes().iter())
      .chain(self.blue_y.to_be_bytes().iter())
      .map(|&v| v)
      .collect()
  }
}
