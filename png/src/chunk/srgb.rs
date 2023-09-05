use std::fmt;
use std::fmt::Display;
use crate::ChunkRawBytes;

/// 0: Perceptual
/// 1: Relative colorimetric
/// 2: Saturation
/// 3: Absolute colorimetric
pub struct ChunkSRGB(u8);

impl Display for ChunkSRGB {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    match self.0 {
      0 => write!(f, "Perceptual"),
      1 => write!(f, "Relative colorimetric"),
      2 => write!(f, "Saturation"),
      3 => write!(f, "Absolute colorimetric"),
      _ => write!(f, "Unknown")
    }
  }
}

impl ChunkRawBytes for ChunkSRGB {
  fn as_bytes(&self) -> Vec<u8> {
    vec![self.0]
  }
}
