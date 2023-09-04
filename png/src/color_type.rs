use std::fmt;
use std::fmt::Display;

use crate::PngError;

/// Color Type
#[derive(Clone, PartialEq)]
pub enum ColorType {
  /// Color Type: 0 <br/>
  /// Allowed Bit Depths: 1, 2, 4, 8, 16
  Grayscale,
  /// Color Type: 2 <br/>
  /// Allowed Bit Depths: 8, 16
  Rgb,
  /// Color Type: 3 <br/>
  /// Allowed Bit Depths: 1, 2, 4, 8
  PaletteIndex,
  /// Color Type: 4 <br/>
  /// Allowed Bit Depths: 8, 16
  GrayscaleWithAlpha,
  /// Color Type: 6 <br/>
  /// Allowed Bit Depths: 8, 16
  RgbWithAlpha,
}

impl Display for ColorType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    match self {
      ColorType::Grayscale => write!(f, "Grayscale"),
      ColorType::Rgb => write!(f, "RGB"),
      ColorType::PaletteIndex => write!(f, "Palette Index"),
      ColorType::GrayscaleWithAlpha => write!(f, "Grayscale(with alpha)"),
      ColorType::RgbWithAlpha => write!(f, "RGBA"),
    }
  }
}

impl TryFrom<u8> for ColorType {
  type Error = PngError;

  fn try_from(v: u8) -> Result<Self, <Self as TryFrom<u8>>::Error> {
    match v {
      0 => Ok(ColorType::Grayscale),
      2 => Ok(ColorType::Rgb),
      3 => Ok(ColorType::PaletteIndex),
      4 => Ok(ColorType::GrayscaleWithAlpha),
      6 => Ok(ColorType::RgbWithAlpha),
      _ => Err(PngError::InvalidColorType)
    }
  }
}

impl Into<u8> for ColorType {
  fn into(self) -> u8 {
    match self {
      ColorType::Grayscale => 0,
      ColorType::Rgb => 2,
      ColorType::PaletteIndex => 3,
      ColorType::GrayscaleWithAlpha => 4,
      ColorType::RgbWithAlpha => 6,
    }
  }
}

impl ColorType {
  pub fn channels(&self) -> u8 {
    match self {
      Self::Grayscale => 1,
      Self::Rgb => 3,
      Self::PaletteIndex => 1,
      Self::GrayscaleWithAlpha => 2,
      Self::RgbWithAlpha => 4,
    }
  }
}

mod tests {
  use super::*;

  #[test]
  fn test_try_from() {
    assert_eq!(ColorType::try_from(0).is_ok(), true);
    assert_eq!(ColorType::try_from(2).is_ok(), true);
    assert_eq!(ColorType::try_from(3).is_ok(), true);
    assert_eq!(ColorType::try_from(4).is_ok(), true);
    assert_eq!(ColorType::try_from(6).is_ok(), true);
    assert_eq!(ColorType::try_from(8).is_err(), true);
  }

  #[test]
  fn test_into() {
    assert_eq!(<ColorType as Into<u8>>::into(ColorType::Grayscale), 0);
    assert_eq!(<ColorType as Into<u8>>::into(ColorType::Rgb), 2);
    assert_eq!(<ColorType as Into<u8>>::into(ColorType::PaletteIndex), 3);
    assert_eq!(<ColorType as Into<u8>>::into(ColorType::GrayscaleWithAlpha), 4);
    assert_eq!(<ColorType as Into<u8>>::into(ColorType::RgbWithAlpha), 6);
  }

  #[test]
  fn test_fmt() {
    assert_eq!(ColorType::Grayscale.to_string(), "Grayscale");
    assert_eq!(ColorType::Rgb.to_string(), "RGB");
    assert_eq!(ColorType::PaletteIndex.to_string(), "Palette Index");
    assert_eq!(ColorType::GrayscaleWithAlpha.to_string(), "Grayscale(with alpha)");
    assert_eq!(ColorType::RgbWithAlpha.to_string(), "RGBA");
  }
}


