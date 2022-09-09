use std::{
    fmt::Display,
    io::{Cursor, Read},
};

use crc::{Crc, CRC_32_ISO_HDLC};

const CHECKSUM_ALG: Crc::<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

use crate::{chunk_type::ChunkType, Error};

#[derive(Debug)]
pub struct Chunk {
    pub length: u32,
    pub chunk_type: ChunkType,
    pub data: Vec<u8>,
    pub crc: u32,
}

#[allow(dead_code)]
impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        // Checksum algorithm

        // The length of the data (very useful comment if you didn't know)
        let length = data.len() as u32;

        // Merge the bytes of the chunk type with the bytes of the data/payload
        // such that the new vec will look like [bytes of chunk type, bytes of payload]
        // and save it to a variable with a helpful name.
        let datae: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .cloned()
            .collect();

        // Calculate the checksum of datae
        let crc = CHECKSUM_ALG.checksum(&datae);

        // Return the chunk
        Chunk {
            length,
            chunk_type,
            data,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, Error> {
        let string = String::from_utf8(self.data.iter().copied().collect())?;

        Ok(string)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        // Merge all of the data into a single byte slice
        self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {

        let mut cursor = Cursor::new(value);
        
        // Read the first 4 bytes (length of a u32) to length
        let mut length: [u8; 4] = [0; 4];
        cursor.read_exact(&mut length)?;
        let length = u32::from_be_bytes(length);

        // Read the chunk type to chunk_type
        let mut chunk_type: [u8; 4] = [0; 4];
        cursor.read_exact(&mut chunk_type)?;
        let chunk_type = ChunkType::new(chunk_type);

        // Read the data to the data vec
        let mut data: Vec<u8> = vec![0u8; length as usize];
        cursor.read_exact(&mut data)?;
        
        // Read the CRC
        let mut crc: [u8; 4] = [0; 4];
        cursor.read_exact(&mut crc)?;
        let crc: u32 = u32::from_be_bytes(crc);

        // Validate the CRC
        let datae: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .cloned()
            .collect();


        if crc != CHECKSUM_ALG.checksum(&datae) {
            return Err(ChunkError::InvalidCRC.into())
        }

        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_bytes())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChunkError {
    #[error("The crc does not match the generated CRC, the data may be corrupted.")]
    InvalidCRC
}


#[cfg(test)]
#[allow(unused_variables)]
mod chunk_tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
