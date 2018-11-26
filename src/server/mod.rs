use std::net::SocketAddr;

mod server;
mod config;
mod tcp_server;
mod udp_server;
mod tcp_client;

pub use self::tcp_server::TcpServer;
pub use self::tcp_client::TcpClient;
pub use self::udp_server::UdpServer;
pub use self::server::Server;
pub use self::config::ServerConfig;

use std::sync::mpsc::{Receiver, Sender};
use net_events::NetEvent;

pub trait ProtocolServer {
    /// Start receiving data.
    fn start_receiving(&mut self, tx: Sender<NetEvent>);
    /// This will send all data.
    fn start_sending(&mut self, rx: Receiver<NetEvent>);
}