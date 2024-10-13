use cql_bindgen::*;
use std::fs;
use std::time::SystemTime;

pub struct ScyllaStorage {
    session: Session,
}

impl ScyllaStorage {
    pub async fn new() -> Result<Self, String> {
        let cluster = Cluster::new("scylla-db", "rudp_keyspace").await?;
        let session = cluster.connect().await?;

        let storage = ScyllaStorage { session };

        // Initialize schema from `schema/schema.cql`
        storage.load_schema().await?;

        Ok(storage)
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

    // Function to load and execute the schema from `schema/schema.cql`
    pub async fn load_schema(&self) -> Result<(), String> {
        // Read the .cql file from the schema directory
        let cql_file_path = "schema/schema.cql";  // Adjust path as necessary
        let cql_script = fs::read_to_string(cql_file_path)
            .map_err(|e| format!("Failed to read schema file: {}", e))?;

        // Execute the CQL schema commands
        self.session
            .execute(&cql_script, ())
            .await
            .map_err(|e| format!("Failed to execute schema: {}", e))?;

        println!("Successfully loaded and executed CQL schema from: {}", cql_file_path);

        Ok(())
    }
}
