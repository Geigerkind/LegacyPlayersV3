use crate::modules::server_exporter::domain_value::MessageType;
use crate::modules::server_exporter::tools::{byte_reader, byte_writer, GUID};
use crate::modules::util::{salt_u32_u64, salt_u64_u64};
use crate::modules::ServerExporter;
use std::sync::mpsc::Sender;

impl ServerExporter {
    pub fn run(&mut self) {
        let context = zmq::Context::new();
        let responder = context.socket(zmq::PULL).unwrap();
        assert!(responder.bind("tcp://0.0.0.0:5690").is_ok());
        println!("Established ZMQ socket!");

        let sender = self.sender_message.as_ref().expect("Sender to be assigned!");
        loop {
            let mut msg = responder.recv_bytes(0).unwrap();

            let api_version = &msg[0];
            if *api_version != 0 {
                continue; // Only API_Version 0 supported
            }

            let message_type = MessageType::from_number(&msg[1]);
            if message_type == MessageType::Undefined {
                continue;
            }

            // Anonymize GUIDs
            match message_type {
                // First
                MessageType::CombatState | MessageType::Power | MessageType::Loot | MessageType::Event | MessageType::Interrupt => {
                    let guid = byte_reader::read_u64(&msg[11..19]);
                    if guid.is_player() {
                        byte_writer::write_u64(&mut msg[11..19], salt_u64_u64(guid));
                    }
                    msg.insert(11, guid.is_player() as u8);
                    msg[2] += 1;
                    send_message(&sender, vec![guid], msg);
                },
                // First 2
                MessageType::MeleeDamage | MessageType::SpellDamage | MessageType::Heal | MessageType::Death | MessageType::SpellCast | MessageType::Threat | MessageType::Summon | MessageType::AuraApplication => {
                    let guid1 = byte_reader::read_u64(&msg[11..19]);
                    let guid2 = byte_reader::read_u64(&msg[19..27]);
                    if guid1.is_player() {
                        byte_writer::write_u64(&mut msg[11..19], salt_u64_u64(guid1));
                    }
                    msg.insert(11, guid1.is_player() as u8);
                    if guid2.is_player() {
                        byte_writer::write_u64(&mut msg[20..28], salt_u64_u64(guid2));
                    }
                    msg.insert(20, guid2.is_player() as u8);
                    msg[2] += 2;
                    send_message(&sender, vec![guid1, guid2], msg);
                },
                // First 3
                MessageType::Dispel | MessageType::SpellSteal => {
                    let guid1 = byte_reader::read_u64(&msg[11..19]);
                    let guid2 = byte_reader::read_u64(&msg[19..27]);
                    let guid3 = byte_reader::read_u64(&msg[27..35]);
                    if guid1.is_player() {
                        byte_writer::write_u64(&mut msg[11..19], salt_u64_u64(guid1));
                    }
                    msg.insert(11, guid1.is_player() as u8);
                    if guid2.is_player() {
                        byte_writer::write_u64(&mut msg[20..28], salt_u64_u64(guid2));
                    }
                    msg.insert(20, guid2.is_player() as u8);
                    if guid3.is_player() {
                        byte_writer::write_u64(&mut msg[29..37], salt_u64_u64(guid3));
                    }
                    msg.insert(29, guid3.is_player() as u8);
                    msg[2] += 3;
                    send_message(&sender, vec![guid1, guid2, guid3], msg);
                },
                // Special Snowflakes
                MessageType::Position => {
                    let guid = byte_reader::read_u64(&msg[20..28]);
                    if guid.is_player() {
                        byte_writer::write_u64(&mut msg[20..28], salt_u64_u64(guid));
                    }
                    msg.insert(20, guid.is_player() as u8);
                    msg[2] += 1;
                    send_message(&sender, vec![guid], msg);
                },
                MessageType::InstancePvpEndRatedArena => {
                    let guid1 = byte_reader::read_u32(&msg[19..23]);
                    let guid2 = byte_reader::read_u32(&msg[19..23]);
                    msg.insert(23, 0);
                    msg.insert(23, 0);
                    msg.insert(23, 0);
                    msg.insert(23, 0);
                    msg.insert(31, 0);
                    msg.insert(31, 0);
                    msg.insert(31, 0);
                    msg.insert(31, 0);
                    byte_writer::write_u64(&mut msg[19..27], salt_u32_u64(guid1));
                    byte_writer::write_u64(&mut msg[27..35], salt_u32_u64(guid2));
                    msg[2] += 8;
                    send_message(&sender, vec![guid1 as u64, guid2 as u64], msg);
                },
                _ => {}, // Ignore
            };
        }
    }
}

fn send_message(sender: &Sender<(Vec<u32>, Vec<u8>)>, guids: Vec<u64>, msg: Vec<u8>) {
    let ids = guids
        .iter()
        .map(|guid| {
            if guid.is_player() {
                return *guid as u32;
            }
            0
        })
        .collect();
    sender.send((ids, msg)).expect("Receiver should be available!");
}
