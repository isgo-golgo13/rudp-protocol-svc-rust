use rudp_protocol_kit::rudp::*;
use std::collections::BTreeMap;
use std::net::UdpSocket;
use std::time::{Duration, SystemTime};
use storage_kit::storage_repository::PacketRepository;

// Struct to manage the buffer of out-of-order packets
struct PacketBuffer {
    buffer: BTreeMap<u32, RudpPacket>, // Stores packets ordered by sequence number
    expected_sequence_num: u32,        // The next expected sequence number
}

impl PacketBuffer {
    fn new() -> Self {
        PacketBuffer {
            buffer: BTreeMap::new(),
            expected_sequence_num: 0,
        }
    }

    // Buffer the packet if out of order
    fn buffer_packet(&mut self, packet: RudpPacket) {
        self.buffer.insert(packet.sequence_num, packet);
    }

    // Process the packet immediately and return true if processed
    fn process_packet(&mut self, packet: RudpPacket, repo: &PacketRepository) -> bool {
        if packet.sequence_num == self.expected_sequence_num {
            self.store_packet(packet, repo);
            self.expected_sequence_num += 1;
            true
        } else {
            false
        }
    }

    // Store the packet in the database
    fn store_packet(&self, packet: RudpPacket, repo: &PacketRepository) {
        let timestamp_received = SystemTime::now();
        println!(
            "Processing packet with sequence number {}: {:?}",
            packet.sequence_num, packet.data
        );
        if let Err(e) = repo.save_packet(
            packet.sequence_num as i32,
            packet.timestamp_sent,
            timestamp_received,
            packet.retry_count > 0,
            packet.retry_count,
        ) {
            eprintln!("Failed to store packet: {}", e);
        }
    }

    // Process buffered packets in sequence once the expected sequence number is reached
    fn process_buffered_packets(&mut self, repo: &PacketRepository) {
        while let Some((sequence_num, packet)) =
            self.buffer.get(&self.expected_sequence_num).cloned()
        {
            self.store_packet(packet, repo);
            self.buffer.remove(&sequence_num);
            self.expected_sequence_num += 1;
        }
    }
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8080")?;
    let packet_repo = PacketRepository::new().expect("Failed to initialize repository");
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;

    let mut packet_buffer = PacketBuffer::new();

    loop {
        // Receive RUDP packet
        if let Ok(Some(packet)) = rudp_recv(&socket) {
            println!(
                "Received packet with sequence number: {}",
                packet.sequence_num
            );

            // If the packet is the expected one, process it immediately
            if packet_buffer.process_packet(packet.clone(), &packet_repo) {
                // Process any buffered packets that can now be processed in order
                packet_buffer.process_buffered_packets(&packet_repo);
            } else if packet.sequence_num > packet_buffer.expected_sequence_num {
                // Packet is out of order, so buffer it
                println!(
                    "Buffering out-of-order packet with sequence number {}",
                    packet.sequence_num
                );
                packet_buffer.buffer_packet(packet);
            } else {
                // Duplicate or old packet, ignore it
                println!(
                    "Ignoring duplicate or old packet with sequence number {}",
                    packet.sequence_num
                );
            }

            // Send ACK
            let ack_packet = RudpPacket {
                sequence_num: packet.sequence_num,
                ack: true,
                data: Vec::new(),
                retry_count: 0,
                timestamp_sent: packet.timestamp_sent,
            };

            rudp_send(&socket, &ack_packet, "127.0.0.1:0")?;
        }
    }
}
