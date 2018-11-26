use laminar::error::NetworkError;
use laminar::{Event, Packet};

/// Net event which occurred on the network.
pub enum NetEvent
{
    /// Event containing an packet with received data.
    Packet(Packet),
    /// Broad cast message.
    BroadCast { data: Box<[u8]>},
    /// Event containing error that has occurred in the network.
    Error(NetworkError),
    /// Events that can happen with an client.
    ClientEvent(ClientEvent),
    /// Empty event.
    Empty
}

pub enum ClientEvent {
    Connected,
    Disconnected,
    Timedout,
    None,
}

impl From<Event> for ClientEvent{
    fn from(event: Event) -> Self {
        match event {
            Event::Connected(_) => { ClientEvent::Connected },
            Event::Disconnected(_) => { ClientEvent::Disconnected },
            Event::TimedOut(_) => { ClientEvent::Timedout },
            Event::QualityChange { .. } => { ClientEvent::None },
        }
    }
}