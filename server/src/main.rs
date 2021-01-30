use anyhow::Result;
use laminar::{Packet, Socket, SocketEvent};
use legion::*;
use log::{info, LevelFilter};
use simplelog::{Config, TermLogger, TerminalMode};
use std::thread;

use zombenum_shared::*;

const SERVER: &str = "0.0.0.0:14191";

fn main() -> Result<()> {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed)?;

    let mut world = World::default();

    info!("Starting server");

    let mut socket = Socket::bind(SERVER)?;
    let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
    let _thread = thread::spawn(move || socket.start_polling());

    loop {
        if let Ok(event) = receiver.recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    let msg = packet.payload();

                    if msg == b"Bye!" {
                        break;
                    }

                    let msg = String::from_utf8_lossy(msg);
                    let ip = packet.addr().ip();

                    if msg == "new_player" {
                        world.push((Position { x: 0.0, y: 0.0 }, Player { id: 0 }));

                        let registry = get_registry();
                        let components =
                            bincode::serialize(&world.as_serializable(any(), &registry))?;
                        sender.send(Packet::reliable_ordered(
                            packet.addr(),
                            components,
                            Some(0u8),
                        ))?;

                        let final_message = "done".as_bytes().to_vec();
                        sender.send(Packet::reliable_ordered(
                            packet.addr(),
                            final_message,
                            Some(0u8),
                        ))?;
                    }

                    println!("Received {:?} from {:?}", msg, ip);

                    sender
                        .send(Packet::reliable_unordered(
                            packet.addr(),
                            "Copy that!".as_bytes().to_vec(),
                        ))
                        .expect("This should send");
                }
                SocketEvent::Connect(address) => {
                    info!("Client connected: {}", address);
                }
                SocketEvent::Disconnect(address) => {
                    info!("Client disconnected: {}", address);
                }
                SocketEvent::Timeout(address) => {
                    info!("Client timed out: {}", address);
                }
            }
        }
    }

    Ok(())
}
