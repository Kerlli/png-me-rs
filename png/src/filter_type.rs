use std::fmt;
use std::fmt::Display;

use crate::PngError;

/// Filter Type
#[derive(Clone, PartialEq)]
pub enum FilterType {
  None,
  Sub,
  Up,
  Average,
  Paeth,
}

impl Display for FilterType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    match self {
      FilterType::None => write!(f, "None"),
      FilterType::Sub => write!(f, "Sub"),
      FilterType::Up => write!(f, "Up"),
      FilterType::Average => write!(f, "Average"),
      FilterType::Paeth => write!(f, "Paeth"),
    }
  }
}

impl TryFrom<u8> for FilterType {
  type Error = PngError;

  fn try_from(v: u8) -> Result<Self, <Self as TryFrom<u8>>::Error> {
    match v {
      0 => Ok(FilterType::None),
      1 => Ok(FilterType::Sub),
      2 => Ok(FilterType::Up),
      3 => Ok(FilterType::Average),
      4 => Ok(FilterType::Paeth),
      _ => Err(PngError::InvalidFilterType)
    }
  }  
}

impl Into<u8> for FilterType {
  fn into(self) -> u8 {
    match self {
      FilterType::None => 0,
      FilterType::Sub => 1,
      FilterType::Up => 2,
      FilterType::Average => 3,
      FilterType::Paeth => 4,
    }
  }
}


