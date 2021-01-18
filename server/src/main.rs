use anyhow::Result;
use log::{LevelFilter, info};
use simplelog::{Config, TermLogger, TerminalMode};
use std::net::SocketAddr;
use naia_server_socket::{find_my_ip_address, LinkConditionerConfig, Packet, ServerSocket};

const SERVER_PORT: u16 = 14191;
const PING_MSG: &str = "ping";
const PONG_MSG: &str = "pong";

fn main() -> Result<()> {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed)?;

    info!("Starting server");
    smol::block_on(async {
        let current_ip_address = "0.0.0.0".parse()?; //find_my_ip_address().expect("can't find ip address");
        let current_socket_address = SocketAddr::new(current_ip_address, SERVER_PORT);

        let mut server_socket = ServerSocket::listen(current_socket_address)
            .await
            .with_link_conditioner(&LinkConditionerConfig::good_condition());

        let mut sender = server_socket.get_sender();

        loop {
            match server_socket.receive().await {
                Ok(packet) => {
                    let address = packet.address();
                    let message = String::from_utf8_lossy(packet.payload());
                    info!("Server recv <- {}: {}", address, message);

                    if message.eq(PING_MSG) {
                        let to_client_message: String = PONG_MSG.to_string();
                        info!("Server send -> {}: {}", address, to_client_message);
                        sender
                            .send(Packet::new(address, to_client_message.into_bytes()))
                            .await
                            .expect("send error");
                    }
                }
                Err(error) => {
                    info!("Server Error: {}", error);
                }
            }
        }
    })
}
