// Text chunks: iTxt, tExt, zTXt

use std::convert::TryFrom;
use std::fmt;
use std::fmt::Display;

use crate::{ChunkRawBytes, error::PngError};

const NULL_SEPARATOR: u8 = 0x0;

/// Chunk: tEXt
pub struct ChunkTextual {
  /// 1 - 79 bytes
  keyword: String,
  /// From zero up to the maximum permissible chunk size less the length of the keyword and separator
  text: Option<String>,
}

impl TryFrom<&[u8]> for ChunkTextual {
  type Error = PngError;

  fn try_from(bytes: &[u8]) -> Result<Self, <Self as TryFrom<&[u8]>>::Error> {
    let Some(null_separator_pos) = bytes.iter().position(|&v| v == NULL_SEPARATOR) else {
      return Err(PngError::ChunkParseError);
    };
    // keyword at least takes 1 byte
    if null_separator_pos < 1 {
      return Err(PngError::ChunkParseError);
    }

    let keyword = String::from_utf8(bytes[0..null_separator_pos].iter().map(|&v| v).collect())?;
    let text: Option<String> = match bytes.get(null_separator_pos + 1..) {
      Some(s) => {
        let v = String::from_utf8(s.iter().map(|&v| v).collect())?;
        Some(v)
      },
      None => None,
    };

    Ok(Self { keyword, text })
  }
}

impl ChunkRawBytes for ChunkTextual {
  fn as_bytes(&self) -> Vec<u8> {
    let text = self.text.clone().unwrap_or(String::from(""));
    
    self.keyword.bytes()
      .chain(Some(NULL_SEPARATOR))
      .chain(text.bytes())
      .collect()
  }
}

impl Display for ChunkTextual {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    write!(f, "ChunkTextual {{Keyword: {}, Text: {:?}}}", self.keyword, self.text)
  }
}

impl ChunkTextual {
  pub fn keyword(&self) -> &str {
    &self.keyword
  }

  pub fn text(&self) -> &Option<String> {
    &self.text
  }
}

/// Chunk: zTXt
pub struct ChunkTextualCompressed {
  /// 1 - 79 bytes
  keyword: String,
  /// 0: deflate/inflate compression
  compression_method: u8,
  compressed_text: Vec<u8>,
}

impl TryFrom<&[u8]> for ChunkTextualCompressed {
  type Error = PngError;

  fn try_from(bytes: &[u8]) -> Result<Self, <Self as TryFrom<&[u8]>>::Error> {
    todo!()
  }
}

impl ChunkRawBytes for ChunkTextualCompressed {
  fn as_bytes(&self) -> Vec<u8> {
    self.keyword.bytes()
      .chain(Some(NULL_SEPARATOR))
      .chain(Some(self.compression_method))
      .chain(self.compressed_text.iter().map(|&v| v))
      .collect()
  }
}

impl Display for ChunkTextualCompressed {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    todo!()
  }
}

impl ChunkTextualCompressed {
  // todo 
}

/// Chunk: iTXt
pub struct ChunkTextualInternational {
  /// 1 - 79 bytes
  keyword: String,
  /// null sp before this <br/>
  /// 0: uncompressed, 1: compressed
  compression_flag: u8,
  /// Ignored if compression flag is 0
  compression_method: u8,
  language_tag: Vec<u8>,
  /// null sp before this
  /// Implied by the chunk length
  translated_keyword: Vec<u8>,
  /// null sp before this
  text: Vec<u8>,
}

impl TryFrom<&[u8]> for ChunkTextualInternational {
  type Error = PngError;

  fn try_from(bytes: &[u8]) -> Result<Self, <Self as TryFrom<&[u8]>>::Error> {
    todo!()
  }
}

impl ChunkRawBytes for ChunkTextualInternational {
  fn as_bytes(&self) -> Vec<u8> {
    self.keyword.bytes()
      .chain(Some(NULL_SEPARATOR))
      .chain(Some(self.compression_flag))
      .chain(Some(self.compression_method))
      .chain(self.language_tag.iter().map(|&v| v))
      .chain(Some(NULL_SEPARATOR))
      .chain(self.translated_keyword.iter().map(|&v| v))
      .chain(Some(NULL_SEPARATOR))
      .chain(self.text.iter().map(|&v| v))
      .collect()
  }
}

impl Display for ChunkTextualInternational {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    todo!()
  }
}

impl ChunkTextualInternational {
  // todo
}


