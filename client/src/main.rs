use std::thread;

use anyhow::Result;
use laminar::{Packet, Socket, SocketEvent};
use legion::*;
use log::{info, LevelFilter};
use macroquad::prelude::*;
use simplelog::{Config, TermLogger, TerminalMode};

use zombenum_shared::*;

const SERVER: &str = "127.0.0.1:14191";

fn window_conf() -> Conf {
    Conf {
        window_title: "Zombenum".to_string(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    run().await.expect("Error in run()");
}

async fn run() -> Result<()> {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed)?;

    info!("Starting client");
    let world = World::default();

    let mut socket = Socket::bind_any()?;
    info!("Bound on {}", socket.local_addr()?.to_string());

    let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
    let _thread = thread::spawn(move || socket.start_polling());

    info!("Server is {}", SERVER);
    let server = SERVER.parse()?;

    sender.send(Packet::reliable_unordered(server, "new_player".into()))?;
    let state = receiver.recv();

    loop {
        if is_key_down(KeyCode::Escape) {
            std::process::exit(0);
        }

        if is_key_pressed(KeyCode::Enter) {
            sender.send(Packet::reliable_unordered(server, "lol".into()))?;
        }

        let mut move_vec = glam::f32::Vec2::default();
        if is_key_pressed(KeyCode::W) {
            move_vec += glam::f32::vec2(0.0, 1.0)
        }

        sender.send(Packet::reliable_unordered(server, "lol".into()))?;

        if let Ok(event) = receiver.try_recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    dbg!(packet.addr(), server);
                    if packet.addr() == server {
                        println!("Server sent: {}", String::from_utf8_lossy(packet.payload()));
                    } else {
                        println!("Unknown sender.");
                    }
                }
                SocketEvent::Timeout(_) => {}
                _ => (),
            }
        }

        clear_background(WHITE);
        next_frame().await
    }
}
