use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum PngError {
  InvalidBitDepth,
  InvalidColorType,
  InvalidCompressionMehtod,
  InvalidFilterMethod,
  InvalidFilterType,
  InvalidHeader,
  InvalidInterlaceMethod,
  ChunkCrcMismatch,
  ChunkNotFoundError,
  ChunkParseError,
  ChunksIsEmptyError,
  ChunkTypeParseError(String),
  IndexOutOfBounds,
  IoError(std::io::Error),
  StringFromUtf8Error(std::string::FromUtf8Error),
}

impl std::error::Error for PngError {}

impl Display for PngError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    match self {
      PngError::InvalidBitDepth => write!(f, "Invalid bit depth"),
      PngError::InvalidColorType => write!(f, "Invalid color type"),
      PngError::InvalidCompressionMehtod => write!(f, "Invalid compression method"),
      PngError::InvalidFilterMethod => write!(f, "Invalid filter method"),
      PngError::InvalidFilterType => write!(f, "Invalid filter type"),
      PngError::InvalidHeader => write!(f, "Invalid PNG header"),
      PngError::InvalidInterlaceMethod => write!(f, "Invalid interlace method"),
      PngError::ChunkCrcMismatch => write!(f, "Chunk crc mismatch"),
      PngError::ChunkNotFoundError => write!(f, "Chunk not found"),
      PngError::ChunkParseError => write!(f, "Chunk parse error"),
      PngError::ChunksIsEmptyError => write!(f, "There're no chunks left"),
      PngError::ChunkTypeParseError(err) => write!(f, "Chunk type parse error: {}", err),
      PngError::IndexOutOfBounds => write!(f, "Index out of bounds"),
      PngError::IoError(err) => write!(f, "Io error: {}", err),
      PngError::StringFromUtf8Error(err) => write!(f, "Convert to utf-8 string error: {}", err)
    }
  }
}

impl From<std::io::Error> for PngError {
  fn from(err: std::io::Error) -> Self { Self::IoError(err) }
}

impl From<std::string::FromUtf8Error> for PngError {
  fn from(err: std::string::FromUtf8Error) -> Self { Self::StringFromUtf8Error(err) }
}
