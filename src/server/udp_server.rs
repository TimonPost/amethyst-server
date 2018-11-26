//! All UDP reflated logic for getting and sending data out to the other side.

use std::thread;
use std::net::SocketAddr;
use std::io::{Error, Result, ErrorKind};
use std::sync::mpsc::{self, Receiver, Sender};

use net_events::{NetEvent, ClientEvent};
use laminar::net::UdpSocket;
use laminar::NetworkConfig;
use super::{ProtocolServer, ServerConfig};

type MessageSender = Option<Sender<String>>;
type MessageReceiver = Option<Receiver<String>>;

pub struct UdpServer {
    socket: UdpSocket,
}

impl UdpServer {
    pub fn new(addr: SocketAddr) -> UdpServer
    {
        let mut socket = UdpSocket::bind(&addr, NetworkConfig::default()).unwrap();

        UdpServer { socket }
    }
}

impl ProtocolServer for UdpServer
{
    // 1. Receive data from socket.
    // 2. Analyze data
    // 3. Check if there are events from laminar.
    // 3. Notify the receiver with the received data over send channel.
    fn start_receiving(&mut self, tx: Sender<NetEvent>) {
        loop {
            // try receiving
            let result = self.socket.recv();

            match result {
                Ok(Some(packet)) => {
                    tx.send(NetEvent::Packet(packet));
                }
                Ok(None) => {
                    tx.send(NetEvent::Empty);
                }
                Err(e) => {
                    tx.send(NetEvent::Error(e));
                }
            };

            self.socket.events().into_iter().map(|e| { println!("Event");  tx.send(NetEvent::ClientEvent(ClientEvent::from(e))); });
        }
    }

    // 1. Receive all packets from the channel with packets to send.
    // 2. Sent the packet to a specific client.
    fn start_sending(&mut self, rx: Receiver<NetEvent>) {
        loop {
            match rx.recv() {
                Ok(packet) => {
                    match packet {
                        NetEvent::Packet(packet) => {
                            self.socket.send(&packet);
                        },
                        NetEvent::BroadCast { .. } => {},
                        _ => {}
                    }
                },
                Err(e) => {}
            }
        }
    }
}
