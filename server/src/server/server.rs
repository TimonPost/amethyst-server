use laminar::{error::{NetworkError, NetworkResult}, Packet, NetworkConfig, DeliveryMethod};
use laminar::net::{UdpSocket, VirtualConnection};
use net_events::NetEvent;
use super::{ServerConfig, UdpServer, TcpServer, ProtocolServer};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;
use std::net::SocketAddr;

// Server where clients can connect to.
pub struct Server {
    config: ServerConfig,

    /// handle that should be used fro reading the received udp packets.
    receive_udp: Option<Receiver<NetEvent>>,
    /// handle that should be used for sending udp packets.
    send_upd: Option<Sender<NetEvent>>,
    /// handle that should be used for reading the received tcp packets.
    receive_tcp: Option<Receiver<NetEvent>>,
    /// handle that should be used for reading the received tcp packets.
    send_tcp: Option<Sender<NetEvent>>,

    /// thread handle that receives tcp packets
    pub receive_tcp_handle: Option<JoinHandle<()>>,
    /// thread handle that sends all packets on sent with `send_tcp`
    pub send_tcp_handle: Option<JoinHandle<()>>,
    /// thread handle that receives udp packets
    pub receive_upd_handle: Option<JoinHandle<()>>,
    /// thread handle that sends all packets on sent with `send_udp`
    pub sent_upd_handle: Option<JoinHandle<()>>,
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        Server { config: config.clone(),
            receive_udp: None, send_upd: None,
            receive_upd_handle: None, sent_upd_handle: None,
            receive_tcp: None, send_tcp: None,
            receive_tcp_handle: None, send_tcp_handle: None,
        }
    }

    pub fn run(&mut self) {
        if self.config.enable_tcp {

            // setup receiving tcp thread.
            let (tx, rx) = mpsc::channel();
            let mut tcp_server = TcpServer::new(self.config.clone(), tx);
            self.receive_tcp = Some(rx);

            // setup sending tcp thread
            let (tx, rx) = mpsc::channel();

            self.send_tcp_handle = Some(thread::spawn(move || {
                tcp_server.start_sending(rx);
            }));

            self.send_tcp = Some(tx);
        }

        if self.config.enable_udp {

            // setup the udp server who receives packets.
            let (tx, rx) = mpsc::channel();

            let mut udp_receiver = UdpServer::new(self.config.udp_receive_addr);
            self.receive_upd_handle = Some(thread::spawn(move || {
                udp_receiver.start_receiving(tx);
            }));

            self.receive_udp = Some(rx);

            // setup the udp server that sends packets.
            let (tx, rx) = mpsc::channel();
            let mut udp_sender = UdpServer::new(self.config.udp_send_addr);

            self.sent_upd_handle = Some(thread::spawn(move || {
                udp_sender.start_sending(rx);
            }));

            self.send_upd = Some(tx);
        }
    }

    /// Get all received packets from either TCP and UDP.
    pub fn get_events(&self) -> Vec<NetEvent> {

        let mut vec = Vec::new();

        if let Some(receiver) = &self.receive_udp {
            while let Ok(value) = receiver.try_recv() {
                vec.push(value);
            }
        }

        if let Some(receiver) = &self.receive_tcp {
            while let Ok(value) = receiver.try_recv() {
                vec.push(value);
            }
        }

        vec
    }

    pub fn shutdown(&mut self) {
        // 1. shutdown all connections udp/tcp
    }

    pub fn broad_cast_tcp(&mut self, payload: &[u8], addr: SocketAddr) -> NetworkResult<()>
    {
        // 1. Loop tcp clients.
        // 2. Send data to all clients.
        unimplemented!()
    }

    pub fn broad_cast_upd(&mut self, payload: &[u8], addr: SocketAddr, delivery_method: DeliveryMethod) -> NetworkResult<()>
    {
        // 1. Loop udp clients.
        // 2. Send data to all clients.
        unimplemented!()
    }

    pub fn send_udp(&mut self, packet: Packet) -> NetworkResult<()>
    {
        unimplemented!()
    }

    pub fn send_tcp(&mut self, packet: Packet) -> NetworkResult<()>
    {
        unimplemented!()
    }

    fn find_client_by_addr<'a>(addr: &SocketAddr) -> Option<&'a mut VirtualConnection> {
        unimplemented!()
    }

    fn find_client_by_id<'a>(client_id: u64) -> Option<&'a mut VirtualConnection> {
        unimplemented!()
    }

    fn disconnect<'a>(client_id: u64) -> Option<&'a mut VirtualConnection> {
        unimplemented!()
    }
}