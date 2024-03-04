use bytes::Bytes;
use packet::ip::v4::Packet;
use tokio::sync::{broadcast, mpsc};
use tracing::info;

use crate::config::{MidiPeerSection, Peer};
use crate::streams::handle_stream;

pub async fn connect_midi(
    peer: MidiPeerSection,
    broadcast_rx: broadcast::Receiver<Packet<Bytes>>,
    mpsc_tx: mpsc::Sender<Bytes>,
) -> anyhow::Result<()> {
    todo!();
    info!("Connected to {}.", &peer.common.path);

    //handle_stream(stream, broadcast_rx, mpsc_tx, Peer::Midi(peer)).await?;

    Ok(())
}