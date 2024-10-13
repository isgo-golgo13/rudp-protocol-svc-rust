use cql_bindgen::*;
use std::time::SystemTime;

pub struct ScyllaStorage {
    session: Session,
}

impl ScyllaStorage {
    pub async fn new() -> Result<Self, String> {
        let cluster = Cluster::new("scylla-db", "rudp_keyspace").await?;
        Ok(Self {
            session: cluster.connect().await?,
        })
    }

    pub async fn save_packet(
        &self,
        packet_id: i32,
        timestamp_sent: u64,
        timestamp_received: u64,
        retried: bool,
        retry_count: u32,
    ) -> Result<(), String> {
        let query = "INSERT INTO packet_logs (primary_key_id, packet_id, timestamp_sent, timestamp_received, retried, retried_count)
                     VALUES (uuid(), ?, ?, ?, ?, ?);";
        self.session
            .execute(query, (packet_id, timestamp_sent, timestamp_received, retried, retry_count))
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
