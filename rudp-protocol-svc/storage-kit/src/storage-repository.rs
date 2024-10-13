use crate::storage::ScyllaStorage;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct PacketRepository {
    storage: ScyllaStorage,
}

impl PacketRepository {
    pub async fn new() -> Result<Self, String> {
        let storage = ScyllaStorage::new().await?;
        Ok(Self { storage })
    }

    pub async fn save_packet(
        &self,
        packet_id: i32,
        timestamp_sent: u64,
        retried: bool,
        retry_count: u32,
    ) -> Result<(), String> {
        let timestamp_received = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        self.storage
            .save_packet(packet_id, timestamp_sent, timestamp_received, retried, retry_count)
            .await
    }
}
