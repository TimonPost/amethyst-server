use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::io::{Result, Error, ErrorKind, Read};
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use net_events::NetEvent;
use laminar::{error::NetworkError, Packet, DeliveryMethod};
use log::error;

/// A remote client connected via a TcpStream
#[derive(Debug)]
pub struct TcpClient {
//    reader: BufReader<TcpStream>,
//    writer: BufWriter<TcpStream>,
    raw_stream: TcpStream,

    tx: Sender<Vec<u8>>,
    rx: Option<Receiver<Vec<u8>>>,

    server_channel: Sender<NetEvent>
}

impl TcpClient {
    /// Creates and returns a new TcpClient. It makes a few references to the raw stream and wraps them in BufReader and BufWriter for convenience.
    pub fn new(stream: TcpStream, sender: Sender<NetEvent>) -> Result<TcpClient> {
//        let reader = BufReader::new(stream.try_clone()?);
//        let writer = BufWriter::new(stream.try_clone()?);
        let channel = mpsc::channel();

        Ok(TcpClient {
//            reader,
//            writer,
            raw_stream: stream,
            tx: channel.0,
            rx: Some(channel.1),
            server_channel: sender
        })
    }

    /// Sets up the background loop that waits for data to be received on the rx channel that is meant to be sent to the remote client, then enters a loop to watch for input *from* the remote endpoint.
    pub fn run(client: Arc<Mutex<TcpClient>>) -> Result<()> {
        TcpClient::start_recv(client.clone())?;
        let mut data = vec![0 as u8; 1024];
        loop {
            if let Ok(mut l) = client.lock() {
                match l.raw_stream.read(&mut data) {
                    Ok(size) => {
                        let received = &data[0..size];
                        println!("{:?}", received);
                        if received.len() == 0 {
                            l.server_channel.send(NetEvent::Packet(Packet::new(l.raw_stream.peer_addr().unwrap(), received.to_vec().into_boxed_slice(), DeliveryMethod::ReliableOrdered)));
                        }
                    }
                    Err(e) => {
                        error!("Error receiving: {:?}", e);
                        l.server_channel.send(NetEvent::Error(NetworkError::from(e)));
                    }
                }
            } else {
                error!("Error receiving:" );
                return Err(Error::new(ErrorKind::Other, ""))?
            }
        }
    }

    fn start_recv(client: Arc<Mutex<TcpClient>>) -> Result<()> {
        if let Ok(mut l) = client.lock() {
            match l.outgoing_loop() {
                Ok(_) => Ok(()),
                Err(e) => {
                    return Err(e);
                }
            }
        } else {
            return Err(Error::new(ErrorKind::Other, ""))?
        }
    }

    pub fn send(&mut self, data: &[u8]) {
        self.tx.send(data.to_vec());
    }

    // Starts a thread that watches for incoming messages from the application and writes it to the client
    fn outgoing_loop(&mut self) -> Result<JoinHandle<()>> {
        let mut writer = match self.raw_stream.try_clone() {
            Ok(w) => w,
            Err(_e) => {
                return Err(Error::new(ErrorKind::Other, ""))?
            }
        };

        // We use take here because we can only have one copy of a receiver and we want to the thread to own it
        // The match is used because `std::option::NoneError` is still on nightly
        let rx = match self.rx.take() {
            Some(rx) => rx,
            None => {
                return Err(Error::new(ErrorKind::Other, ""))
            }
        };

        let server_channel = self.server_channel.clone();

        Ok(thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(data) => {
                        match writer.write_all(&data) {
                            Ok(_) => {}
                            Err(_e) => {}
                        };
                        match writer.flush() {
                            Ok(_) => {}
                            Err(_e) => {}
                        }
                    }
                    Err(_e) => {}
                }
            }
        }))
    }
}