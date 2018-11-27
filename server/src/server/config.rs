use std::time::Duration;
use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub tickrate: Duration,
    /// Address at which the UDP packets will be send from to the client.
    pub udp_send_addr: SocketAddr,
    /// Address at which the UDP server will listen for incoming packets.
    pub udp_receive_addr: SocketAddr,
    /// TCP address at which the server will listen for incoming packets.
    pub tcp_addr: SocketAddr,
    /// Specifies whether the server should listen for TCP packets.
    pub enable_tcp: bool,
    /// Specifies whether the server should receive UDP packets.
    pub enable_udp: bool,
}

impl Default for ServerConfig
{
    fn default() -> Self {
        ServerConfig {
            tickrate: Duration::from_millis(100),
            udp_send_addr: "127.0.0.1:12341".parse().unwrap(),
            udp_receive_addr: "127.0.0.1:12342".parse().unwrap(),
            tcp_addr: "127.0.0.1:12343".parse().unwrap(),
            enable_tcp: true,
            enable_udp: true
        }
    }
}