use std::io::prelude::*;
use std::net::{TcpStream, TcpListener, UdpSocket};

use specs::{System, ReadStorage};

use components::common::Position;


// TODO: this "broadcasts" only when something connects
// to the tcp listener, which is kinda dumb
pub struct TcpBroadcast;

impl<'a> System <'a> for TcpBroadcast {
    type SystemData = (ReadStorage<'a, Position>);

    fn run(&mut self, pos: Self::SystemData) {
        use specs::Join;

        let listener = TcpListener::bind("127.0.0.1:8008")
            .expect("couldn't bind to adress!");

        let mut stream = match listener.accept() {
            Ok((_stream, _addr)) => _stream,
            Err(e) => panic!(format!("couldn't accept: {:?}", e)),
        };

        for pos in (&pos).join() {
            let p = json!({
                "type": "entity", // TODO
                "x": pos.x,
                "y": pos.y,
                "z": pos.z,
            });

            let _ = stream.write(
                &format!("{:?}", p.to_string()).as_bytes()
                );
        }
    }
}
