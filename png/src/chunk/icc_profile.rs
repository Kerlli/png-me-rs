use std::fmt;
use std::fmt::Display;
use crate::ChunkRawBytes;

pub struct ChunkICCProfile {
  /// 1 - 79 bytes
  profile_name: String,
  compression_method: u8,
  compression_profile: Vec<u8>,
}

impl Display for ChunkICCProfile {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    write!(f, "Profile Name: {}\nCompression method: Deflate", self.profile_name)
  }
}

impl TryFrom<&Vec<u8>> for ChunkICCProfile {
  type Error = &'static str;

  fn try_from(bytes: &Vec<u8>) -> Result<Self, <Self as TryFrom<&Vec<u8>>>::Error> {
    let profile_name_end_position = bytes.iter().position(|&v| v == 0);

    if profile_name_end_position.is_none() {
      return Err("Cannot determine null separator position")
    }

    let end = profile_name_end_position.unwrap();

    if end == 0 {
      return Err("Invalid null separator position at 0")
    }

    if end > 79 {
      return Err("Profile name out of range")
    }

    let profile_name = String::from_utf8(bytes[0..end].to_vec()).unwrap();

    let compression_method: u8 = *(bytes.get(end + 1).unwrap());

    let compression_profile: Vec<u8> = bytes.iter().skip(end + 1).map(|&v| v).collect();

    Ok(Self {
      profile_name,
      compression_method,
      compression_profile,
    })
  }
}

impl ChunkRawBytes for ChunkICCProfile {
  fn as_bytes(&self) -> Vec<u8> {
    self.profile_name.as_bytes().iter()
      .chain(Some(0).iter())
      .chain(Some(self.compression_method).iter())
      .chain(self.compression_profile.iter())
      .map(|&v| v)
      .collect()
  }
}
