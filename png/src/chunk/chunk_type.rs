use std::str::FromStr;

use super::PngError;

/// Ancillary bit:
/// 0 (uppercase) = critical, 1 (lowercase) = ancillary.
/// 
/// Private bit:
/// 0 (uppercase) = public, 1 (lowercase) = private.
/// 
/// Reserved bit:
/// Must be 0 (uppercase) in files conforming to this version of PNG.
/// 
/// Safe-to-copy bit:
/// 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ChunkType([u8; 4]);

impl std::fmt::Display for ChunkType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    let s: String = self.0.iter().map(|&c| c as char).collect();
    write!(f, "{}", s)
  }
}

// 65 - 90
// 97 - 122

impl TryFrom<[u8; 4]> for ChunkType {
  type Error = PngError;

  fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
    for v in value.iter() {
      if !is_valid_word(*v) {
        return Err(PngError::ChunkTypeParseError)
      }
      // if i == 2 && *v != 0 {
      //   return Err(PngError::ChunkTypeParseError)
      // }
    }

    Ok(Self(value))
  }
}

fn is_valid_word(w: u8) -> bool {
  match w {
    w if w < 65 => false,
    w if w > 90 && w < 97 => false,
    w if w > 122 => false,
    _ => true,
  }
}

fn bit_5_of(val: u8) -> u8 {
  (val >> 5) & 1
}

impl FromStr for ChunkType {
  type Err = PngError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.len() != 4 {
      return Err(PngError::ChunkParseError)
    }

    let mut v: [u8; 4] = [0, 0, 0, 0];

    for (i, b) in s.bytes().enumerate() {
      if !is_valid_word(b) {
        return Err(PngError::ChunkParseError)
      }
      v[i] = b;
    }

    Ok(ChunkType(v))
  }
}

impl ChunkType {
  pub fn bytes(&self) -> [u8; 4] {
    self.0
  }

  pub fn is_valid(&self) -> bool {
    if bit_5_of(self.0[2]) != 0 {
      return false
    }

    true
  }

  pub fn is_critical(&self) -> bool {
    bit_5_of(self.0[0]) == 0
  }

  pub fn is_public(&self) -> bool {
    bit_5_of(self.0[1]) == 0
  }

  pub fn is_reserved_bit_valid(&self) -> bool {
    bit_5_of(self.0[2]) == 0
  }

  pub fn is_safe_to_copy(&self) -> bool {
    bit_5_of(self.0[3]) == 1
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
