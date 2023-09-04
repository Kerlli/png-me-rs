use std::fmt;
use std::fmt::Display;

pub enum Color {
  Grayscale(u8, u8, u8),
  Rgb(u8, u8, u8),
  PaletteIndex(u8, u8, u8, u8),
  GrayscaleA(u8, u8, u8, u8),
  RgbA(u8, u8, u8, u8),
}

impl Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    match self {
      Self::Grayscale(a, b, c) => write!(f, "Grayscale({}, {}, {})", a, b, c),
      Self::Rgb(r, g, b) => write!(f, "RGB({}, {}, {})", r, g, b),
      Self::PaletteIndex(r, g, b, alpha) => write!(f, "Palette({}, {}, {}, {})", r, g, b, alpha),
      Self::GrayscaleA(a, b, c, alpha) => write!(f, "GrayscaleAlpha({}, {}, {}, {})", a, b, c, alpha),
      Self::RgbA(r, g, b, a) => write!(f, "RGBA({}, {}, {}, {})", r, g, b, a),
    }
  }
}
