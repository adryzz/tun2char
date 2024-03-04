use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::types::{CompressionType, EncryptionType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub interface: InterfaceSection,

    #[serde(rename = "peer-char")]
    #[serde(default)]
    pub peer_char: Vec<CharPeerSection>,

    #[serde(rename = "peer-sock")]
    #[serde(default)]
    pub peer_sock: Vec<SockPeerSection>,

    #[serde(rename = "peer-sock-listen")]
    #[serde(default)]
    pub peer_sock_listen: Vec<SockListenPeerSection>,

    #[serde(rename = "peer-midi")]
    #[serde(default)]
    pub peer_midi: Vec<MidiPeerSection>,
}

impl Config {
    pub fn get_all_peers(&self) -> Vec<Peer> {
        let mut vec = Vec::new();
        for c in &self.peer_char {
            vec.push(Peer::Char(c.clone()));
        }

        for s in &self.peer_sock {
            vec.push(Peer::Sock(s.clone()));
        }

        for s in &self.peer_sock_listen {
            vec.push(Peer::SockListen(s.clone()));
        }

        for s in &self.peer_midi {
            vec.push(Peer::Midi(s.clone()));
        }

        vec
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceSection {
    pub address: IpNetwork,
    pub name: String,
    #[serde(rename = "ip-filtering")]
    pub ip_filtering: Option<bool>,
    pub buffer: Option<usize>,
    #[serde(rename = "post-up")]
    pub post_up: Option<String>,
    #[serde(rename = "post-down")]
    pub post_down: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonOptions {
    pub path: String,
    pub allowedips: Vec<IpNetwork>,
    #[serde(default)]
    pub compression: Option<CompressionType>,
    #[serde(default)]
    pub encryption: Option<EncryptionType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharPeerSection {
    #[serde(flatten)]
    pub common: CommonOptions,

    pub speed: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SockPeerSection {
    #[serde(flatten)]
    pub common: CommonOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SockListenPeerSection {
    #[serde(flatten)]
    pub common: CommonOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiPeerSection {
    #[serde(flatten)]
    pub common: CommonOptions,
}

#[derive(Debug, Clone)]
pub enum Peer {
    Char(CharPeerSection),
    Sock(SockPeerSection),
    SockListen(SockListenPeerSection),
    Midi(MidiPeerSection),
}

macro_rules! peer_field {
    ($self:expr, $field:ident) => {
        match $self {
            Peer::Char(c) => &c.$field,
            Peer::Sock(c) => &c.$field,
            Peer::SockListen(c) => &c.$field,
            Peer::Midi(c) => &c.$field,
        }
    };
}

impl Peer {
    pub fn common(&self) -> &CommonOptions {
        peer_field!(self, common)
    }

    pub fn allowed_ips(&self) -> &[IpNetwork] {
        &self.common().allowedips
    }

    pub fn path(&self) -> &str {
        &self.common().path
    }

    pub fn compression(&self) -> CompressionType {
        self.common().compression.unwrap_or(CompressionType::None)
    }

    pub fn encryption(&self) -> EncryptionType {
        self.common().encryption.unwrap_or(EncryptionType::None)
    }
}

pub async fn parse_config() -> anyhow::Result<(Config, Vec<Peer>)> {
    let config_text = tokio::fs::read_to_string("ip2char.toml").await?;
    let config = toml::from_str::<Config>(&config_text)?;
    info!("[0] Read config file.");

    let all_peers = config.get_all_peers();

    if all_peers.is_empty() {
        warn!("Zero peers listed in configuration file!");
    }

    Ok((config, all_peers))
}
