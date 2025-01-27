use bytemuck::from_bytes;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{error, info};
use num_enum::TryFromPrimitive;

use crate::{utils, HEADER_SIZE};

pub const VERSION: u16 = 0;
pub const MARKER_SIZE: usize = 4;
pub const SYNC_MARKER: [u8; MARKER_SIZE] = [0xac, 0xab, 0xc0, 0xde];

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct Header {
    pub marker: [u8; MARKER_SIZE],
    pub version: u16,
    pub packet_length: u16,
    pub compression: CompressionType,
    pub encryption: EncryptionType,
    _reserved: [u8; 6],
}

impl Header {
    pub fn from_slice(slice: &[u8]) -> anyhow::Result<Self> {
        if slice.len() < HEADER_SIZE {
            return Err(IntoErrors::BufferTooSmall.into());
        }
        let marker = *from_bytes::<[u8; MARKER_SIZE]>(&slice[..4]);
        if marker != SYNC_MARKER {
            return Err(IntoErrors::BadSyncMarker.into());
        }
        let version = *from_bytes::<u16>(&slice[4..6]);
        let packet_length = *from_bytes::<u16>(&slice[6..8]);
        let compression = slice[8].try_into()?;
        let encryption = slice[9].try_into()?;
        Ok(Self {
            marker,
            version,
            packet_length,
            compression,
            encryption,
            _reserved: [0; 6],
        })
    }
}

impl From<Header> for [u8; HEADER_SIZE] {
    fn from(val: Header) -> [u8; HEADER_SIZE] {
        let mut buf = [0u8; HEADER_SIZE];
        buf[..4].copy_from_slice(&val.marker);
        buf[4..6].copy_from_slice(&val.version.to_le_bytes());
        buf[6..8].copy_from_slice(&val.packet_length.to_le_bytes());
        buf[8] = val.compression as u8;
        buf[9] = val.encryption as u8;
        buf[10..16].copy_from_slice(&val._reserved);
        buf
    }
}

impl Default for Header {
    fn default() -> Self {
        Self {
            marker: SYNC_MARKER,
            version: VERSION,
            packet_length: 0,
            compression: Default::default(),
            encryption: Default::default(),
            _reserved: Default::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default, TryFromPrimitive)]
#[num_enum(error_type(name = IntoErrors, constructor = IntoErrors::NoSuchVariant))]
#[repr(u8)]
#[serde(rename_all = "lowercase")]
pub enum CompressionType {
    #[default]
    None = 0,
    Zstd = 1,
    #[serde(rename = "zstd-fast")]
    ZstdFast = 2,
    #[serde(rename = "zstd-slow")]
    ZstdSlow = 3,
    Gzip = 4,
}

#[derive(Error, Debug)]
pub enum IntoErrors {
    #[error("no variant exists for integer {0}")]
    NoSuchVariant(u8),

    #[error("Sync marker doesn't match")]
    BadSyncMarker,

    #[error("Buffer too small!")]
    BufferTooSmall,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default, TryFromPrimitive)]
#[num_enum(error_type(name = IntoErrors, constructor = IntoErrors::NoSuchVariant))]
#[repr(u8)]
#[serde(rename_all = "lowercase")]
pub enum EncryptionType {
    #[default]
    None = 0,
}

pub struct PostCommand {
    post_down: Option<String>,
}

impl PostCommand {
    pub fn new(post_up: Option<String>, post_down: Option<String>) -> Self {
        if let Some(up) = &post_up {
            match utils::run_command(up) {
                Ok(_) => info!("post-up: {}", up),
                Err(e) => error!("post-up: {}", e),
            }
        }
        Self { post_down }
    }
}

impl Drop for PostCommand {
    fn drop(&mut self) {
        if let Some(down) = &self.post_down {
            match utils::run_command(down) {
                Ok(_) => info!("post-down: {}", down),
                Err(e) => error!("post-down: {}", e),
            }
        }
    }
}
