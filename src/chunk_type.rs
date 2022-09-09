use std::io::prelude::*;
use std::str::FromStr;

use crate::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct ChunkType {
    pub chunk_type: [u8; 4],
}

#[allow(dead_code)]
impl ChunkType {
    pub fn new(chunk_type: [u8; 4]) -> Self {
        Self { chunk_type }
    }

    pub fn bytes(&self) -> [u8; 4] {
        self.chunk_type
    }
    pub fn is_valid(&self) -> bool {
        self.chunk_type.is_ascii() && self.is_reserved_bit_valid()
    }
    pub fn is_critical(&self) -> bool {
        self.bytes()[0].is_ascii_uppercase()
    }
    pub fn is_public(&self) -> bool {
        self.bytes()[1].is_ascii_uppercase()
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes()[2].is_ascii_uppercase()
    }
    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes()[3].is_ascii_lowercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        // Check if valid ASCII
        if !value.is_ascii() {
            return Err(ChunkTypeError::InvalidASCII.into());
        }

        Ok(Self::new(value))
    }
}

impl FromStr for ChunkType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(ChunkTypeError::InvalidASCII.into());
        }
        if s.len() != 4 {
            return Err(ChunkTypeError::InvalidLength.into());
        }
        
        if s.chars().nth(2).unwrap().is_ascii_digit() {
            return Err(ChunkTypeError::InvalidChar.into());
        }


        let mut buf: [u8; 4] = [0; 4];

        s.as_bytes().read(&mut buf)?;

        Ok(Self::new(buf))
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.chunk_type).unwrap())
    }
}


#[derive(thiserror::Error, Debug)]
pub enum ChunkTypeError {
    #[error("Invalid ASCII code detected in chunk type")]
    InvalidASCII,

    #[error("Chunk types must only be 4 ASCII characters")]
    InvalidLength,

    #[error("Digit found in 3rd char")]
    InvalidChar
}

#[cfg(test)]
#[allow(unused_variables)]
mod chunk_type_tests {
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
        assert!(_are_chunks_equal);
    }
}
