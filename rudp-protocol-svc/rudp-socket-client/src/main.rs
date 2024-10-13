use rudp_protocol_kit::rudp::*;
use tokio::net::UdpSocket;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").await.expect("Could not bind socket");
    let server_addr = "127.0.0.1:8080";
    let mut sequence_num = 0;

    loop {
        // Get current timestamp
        let timestamp_sent = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let payload = format!("Timestamp: {}", timestamp_sent).into_bytes();

        // Create RUDP packet
        let packet = RudpPacket::new(sequence_num, payload, 0);

        // Send the packet
        rudp_send(&socket, &packet, server_addr).await.unwrap();
        sequence_num += 1;

        // Wait 5 seconds before sending the next packet
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
