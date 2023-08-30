use std::fmt;
use std::fmt::Display;
use crate::ChunkDataDecodeable;

#[derive(Copy, Clone)]
struct Palette(u8, u8, u8);

impl Display for Palette {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    write!(f, "Palette: ({}, {}, {})", self.0, self.1, self.2)
  }
}

pub struct ChunkPalette {
  palettes: Vec<Palette>,
}

impl Display for ChunkPalette {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    write!(f, "{}", self.palettes.iter().map(|&p| p.to_string()).collect::<Vec<String>>().join("\n"))
  }
}

impl ChunkDataDecodeable for ChunkPalette {
  fn as_bytes(&self) -> Vec<u8> {
    self.palettes.iter()
      .map(|&p| vec![p.0, p.1, p.2])
      .flatten()
      .collect::<Vec<u8>>()
  }
}
