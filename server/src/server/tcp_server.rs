//! All TCP reflated logic for getting and sending data out to the other side.

use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::thread;
use std::collections::HashMap;
use std::io::{Error, Result, ErrorKind};
use net_events::{NetEvent, ClientEvent};
use super::{ProtocolServer, ServerConfig, TcpClient};
use log::error;
use log::debug;

pub type Connections = Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<TcpClient>>>>>;
type MessageSender = Option<Sender<String>>;
type MessageReceiver = Option<Receiver<String>>;

pub struct TcpServer {
    pub tcp_clients: Connections,
    pub config: ServerConfig
}

impl TcpServer {
    pub fn new(config: ServerConfig, sender: Sender<NetEvent>) -> TcpServer
    {
        let connections = Arc::new(Mutex::new(HashMap::new()));

        let sender = sender.clone();
        let addr = config.tcp_addr;
        let thread_connections = connections.clone();

        thread::spawn(move || {
            let listener = match TcpListener::bind(addr) {
                Ok(l) => l,
                Err(e) => {
                    error!("Error binding to listening socket: {}", e);
                    return;
                }
            };

            // This is a blocking call, so the thread waits here until it gets a new connection
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        // Now we call a function and pass it the stream, and a clone of the connections hash
                        match TcpServer::handle_connection(stream, thread_connections.clone(), sender.clone()) {
                            Ok(c) => {
                                sender.send(NetEvent::ClientEvent(ClientEvent::Connected));
                            }
                            Err(e) => {
                                error!("Error accepting new TCP connection: {}", e);
                            }
                        };
                    }
                    Err(e) => {
                        debug!("Error accepting new TCP stream: {}", e);
                    }
                };
            }
        });

        TcpServer { tcp_clients: connections, config }
    }

    /// This function inserts a reference to the connection into the connections hash
    pub fn handle_connection(stream: TcpStream, connections: Connections, sender: Sender<NetEvent>) -> Result<()> {
        let peer_addr = stream.peer_addr()?;
        let tmp_stream = stream.try_clone()?;
        let tcp_client = Arc::new(Mutex::new(TcpClient::new(stream, sender.clone())?));

        if !connections.is_poisoned() {
            if let Ok(mut locked_connections) = connections.lock() {
                locked_connections.insert(peer_addr, tcp_client.clone());
                // Pass it off to a function to handle setting up the client-specific background threads
                TcpClient::run(tcp_client)?;
                Ok(())
            } else {
                // If we can't get the lock, send a shutdown to the client and they will have to try again
                tmp_stream.shutdown(Shutdown::Both)?;
                Ok(())
            }
        } else {
            tmp_stream.shutdown(Shutdown::Both)?;
            Err(Error::new(ErrorKind::Other, ""))
        }
    }
}

impl ProtocolServer for TcpServer
{
    fn start_receiving(&mut self, tx: Sender<NetEvent>) { }

    fn start_sending(&mut self, rx: Receiver<NetEvent>) {
        loop {
            match rx.recv() {
                Ok(packet) => {
                    match packet {
                        NetEvent::Packet(packet) => {
                            let connections = self.tcp_clients.lock().unwrap();

                            let mx = connections.get(&packet.addr());

                            match mx {
                                None => { println!("Connection does not exists.")},
                                Some(mx) => {
                                    let mut connection = mx.lock().unwrap();
                                    connection.send(packet.payload())
                                },
                            }
                        },
                        _ => {}
                    }
                },
                Err(e) => {}
            }
        }
    }
}
