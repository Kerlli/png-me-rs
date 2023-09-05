use std::fmt;
use std::fmt::Display;
use crate::ChunkRawBytes;

pub struct ChunkGamma(u32);

impl Display for ChunkGamma {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    write!(f, "{}", self.0)
  }
}

impl ChunkRawBytes for ChunkGamma {
  fn as_bytes(&self) -> Vec<u8> {
    self.0.to_be_bytes().iter().map(|&v| v).collect()
  }
}
