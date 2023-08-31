use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum PngError {
  InvalidHeader,
  ChunkCrcMismatch,
  ChunkNotFoundError,
  ChunkParseError,
  ChunksIsEmptyError,
  IndexOutOfBounds,
}

impl std::error::Error for PngError {}

impl Display for PngError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    match self {
      PngError::InvalidHeader => write!(f, "Invalid PNG header"),
      PngError::ChunkCrcMismatch => write!(f, "Chunk crc mismatch"),
      PngError::ChunkNotFoundError => write!(f, "Chunk not found"),
      PngError::ChunkParseError => write!(f, "Chunk parse error"),
      PngError::ChunksIsEmptyError => write!(f, "There're no chunks left"),
      PngError::IndexOutOfBounds => write!(f, "Index out of bounds"),
    }
  }
}
