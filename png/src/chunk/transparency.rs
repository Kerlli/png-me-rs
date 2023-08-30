use crate::ChunkDataDecodeable;

pub struct ChunkTransparency(Vec<u8>);

impl ChunkDataDecodeable for ChunkTransparency {
  fn as_bytes(&self) -> Vec<u8> {
    self.0.to_vec()
  }
}

impl ChunkTransparency {
  pub fn as_indexed_color_bytes(&self) -> Vec<u8> {
    self.as_bytes()
  }

  pub fn as_grayscale_bytes(&self) -> Vec<u16> {
    todo!()
  }
}
