use crate::{Error, Result};

use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    ancillary: u8,
    private: u8,
    reserved: u8,
    safe_to_copy: u8,
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        let chunk_type: [u8; 4] = [
            self.ancillary,
            self.private,
            self.reserved,
            self.safe_to_copy,
        ];
        chunk_type
    }

    pub fn is_valid(&self) -> bool {
        if self.reserved.is_ascii_uppercase()
            && self.ancillary.is_ascii()
            && self.private.is_ascii()
            && self.safe_to_copy.is_ascii()
        {
            return true;
        }
        return false;
    }

    pub fn is_critical(&self) -> bool {
        if self.ancillary.is_ascii_uppercase() {
            return true;
        }
        return false;
    }

    pub fn is_public(&self) -> bool {
        if self.private.is_ascii_uppercase() {
            return true;
        }
        return false;
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        if self.reserved.is_ascii_uppercase() {
            return true;
        }

        return false;
    }

    pub fn is_safe_to_copy(&self) -> bool {
        if self.safe_to_copy.is_ascii_uppercase() {
            return false;
        }
        return true;
    }

    pub fn is_valid_byte(byte: u8) -> bool {
        if byte.is_ascii_alphabetic() {
            return true;
        }

        return false;
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        if !bytes[2].is_ascii_uppercase() {
            return Err("Invalid Byte".into());
        }

        Ok(ChunkType {
            ancillary: bytes[0],
            private: bytes[1],
            reserved: bytes[2],
            safe_to_copy: bytes[3],
        })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.ancillary, self.private, self.reserved, self.safe_to_copy
        )
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let bytes = s.as_bytes();

        for byte in bytes.iter() {
            if !byte.is_ascii_alphabetic() {
                return Err("Invalid Byte".into());
            }
        }

        Ok(ChunkType {
            ancillary: bytes[0],
            private: bytes[1],
            reserved: bytes[2],
            safe_to_copy: bytes[3],
        })
    }
}

fn main() {}

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
    }
}
