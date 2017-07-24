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

        let addr = "127.0.0.1:8008";

        // bind a tcp listener to localhost:8080
        let listener = TcpListener::bind(addr)
            .expect("couldn't bind to address!");

        info!("listening on {}!", addr);

        // we don't seem to need non-blocking execution
        // listener.set_nonblocking(true)
        //     .expect("Cannot set non-blocking!");

        // iterate over incoming connections
        for incoming in listener.incoming() {
            // `incoming` can be a tcp stream, or an error
            // if there is no incoming connection.
            // If we get an error, we keep trying until something
            // connects.
            let mut stream = match incoming {
                Ok(s) => s,
                Err(_) => continue,
            };

            // write some json to the stream
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
}
