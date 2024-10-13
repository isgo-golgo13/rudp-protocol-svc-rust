use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::UdpSocket;

pub const MAX_DATA_SIZE: usize = 512;

#[derive(Serialize, Deserialize, Debug)]
pub struct RudpPacket {
    pub sequence_num: u32,
    pub ack: bool,
    pub data: Vec<u8>,
    pub retry_count: u32,
    pub timestamp_sent: u64,
}

impl RudpPacket {
    pub fn new(sequence_num: u32, data: Vec<u8>, retry_count: u32) -> Self {
        let timestamp_sent = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        RudpPacket {
            sequence_num,
            ack: false,
            data,
            retry_count,
            timestamp_sent,
        }
    }

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        serde_json::from_slice(buf).ok()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

pub async fn rudp_send(
    socket: &UdpSocket,
    packet: &RudpPacket,
    addr: &str,
) -> tokio::io::Result<()> {
    let buf = packet.to_bytes();
    socket.send_to(&buf, addr).await?;
    Ok(())
}

pub async fn rudp_recv(socket: &UdpSocket) -> tokio::io::Result<Option<RudpPacket>> {
    let mut buf = [0; MAX_DATA_SIZE];
    let (len, _addr) = socket.recv_from(&mut buf).await?;
    Ok(RudpPacket::from_bytes(&buf[..len]))
}
