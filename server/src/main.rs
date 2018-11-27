extern crate laminar;
extern crate log;

mod server;
mod net_events;

use net_events::{NetEvent, ClientEvent};
use self::server::{Server, ServerConfig};

use laminar::{Packet, DeliveryMethod, NetworkConfig, Event};
use laminar::net::UdpSocket;

use std::{time::Duration, thread};
use std::net::{SocketAddr, TcpStream};
use std::io::Write;

fn main() {
    // start server.
    let handle = thread::spawn(|| {
        start_server();
    });

    // give the server the ability to start before we send.
    thread::sleep(Duration::from_millis(200));

    // send packets.
    thread::spawn(|| {
        start_client()
    }).join();

    handle.join();
}

fn start_server() {
    let mut server = Server::new(ServerConfig::default());
    server.run();

    println!("Receiving ...");

    // wait for a certina ammount to give some client the time to send this packets. Whereafter we check if
    thread::sleep(Duration::from_millis(10000));

    let mut counter = 0;

    for i in server.get_events() {
        match i  {
            NetEvent::Packet(p) => { /* println!("Packet received!" ); */ },
            NetEvent::BroadCast { .. } => { println!("Broadcast received!" ); },
            NetEvent::Error(e) => { println!("Error: {:?}", e); },
            NetEvent::ClientEvent(e) => {
                match e {
                    ClientEvent::Connected => { println!("Connected!") },
                    ClientEvent::Disconnected => { println!("Disconnected!") },
                    ClientEvent::Timedout => { println!("Timedout!") },
                    ClientEvent::None => { println!("mmmmm!") }
                }
            }
            NetEvent::Empty => { println!("Empty event") },
        }

        counter += 1;
    }

    println!("received: {}", counter);

    server.sent_upd_handle.unwrap().join();
    server.receive_upd_handle.unwrap().join();
}

fn start_client() {
    // send udp packets
    {
        let mut packet = Packet::new("127.0.0.1:12342".parse().unwrap(), vec![0, 1, 2, 3, 4, 5].into_boxed_slice(), DeliveryMethod::ReliableUnordered);

        let addr: SocketAddr = "127.0.0.1:12344".parse().unwrap();
        let mut socket = UdpSocket::bind(addr, NetworkConfig::default()).unwrap();

        println!("Sending UDP data...");
        for i in 0..100 {
            socket.send(&packet);
        }
        println!("Sent all UDP data...");
    }

    // send tcp packets
    {
        let mut stream = TcpStream::connect("127.0.0.1:12343").unwrap();

        println!("Sending TCP data...");
        for i in 0..100 {
            stream.write(vec![1,2,3,4].as_slice()).unwrap();
            stream.flush().unwrap();
        }
        println!("Sent all TCP data...");

        let mut string = String::new();
        std::io::stdin().read_line(&mut string);
    }
}
