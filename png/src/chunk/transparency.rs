use crate::{ChunkRawBytes, color_type::ColorType};

pub struct ChunkTransparency(Vec<u8>);

impl ChunkRawBytes for ChunkTransparency {
  fn as_bytes(&self) -> Vec<u8> {
    self.0.to_vec()
  }
}

impl ChunkTransparency {
  pub fn get_transparency(&self, color_type: &ColorType, at: usize) -> Option<&u8> {
    match color_type {
      ColorType::PaletteIndex => self.0.get(at),
      _ => None,
    }
  }
}
