use std::io::{Cursor, Read};

use bytemuck::from_bytes;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use thiserror::Error;

const VERSION: u16 = 0;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Header {
    pub version: u16,
    pub packet_length: u16,
    pub compression: CompressionType,
    pub encryption: EncryptionType,
    _reserved: [u8; 10],
}

impl Header {
    pub fn from_slice(slice: &[u8]) -> anyhow::Result<Self> {
        let version = *from_bytes::<u16>(&slice[..2]);
        let packet_length = *from_bytes::<u16>(&slice[2..4]);
        let compression = slice[4].try_into()?;
        let encryption = slice[5].try_into()?;
        Ok(Self {
            version,
            packet_length,
            compression,
            encryption,
            _reserved: [0; 10],
        })
    }
}

impl Default for Header {
    fn default() -> Self {
        Self {
            version: VERSION,
            packet_length: 0,
            compression: Default::default(),
            encryption: Default::default(),
            _reserved: Default::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr, Default)]
#[repr(u8)]
pub enum CompressionType {
    #[default]
    None = 0,
}

#[derive(Error, Debug)]
pub enum IntoErrors {
    #[error("no variant exists for integer {0}")]
    NoSuchVariant(u8),
}

impl TryInto<CompressionType> for u8 {
    type Error = IntoErrors;

    fn try_into(self) -> Result<CompressionType, Self::Error> {
        match self {
            0 => Ok(CompressionType::None),
            n => Err(IntoErrors::NoSuchVariant(n)),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr, Default)]
#[repr(u8)]
pub enum EncryptionType {
    #[default]
    None = 0,
}

impl TryInto<EncryptionType> for u8 {
    type Error = IntoErrors;

    fn try_into(self) -> Result<EncryptionType, Self::Error> {
        match self {
            0 => Ok(EncryptionType::None),
            n => Err(IntoErrors::NoSuchVariant(n)),
        }
    }
}
