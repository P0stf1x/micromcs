use std::{io::{Read, Write}, net::TcpStream};

use tracing::{debug, info, trace};

use crate::types;
use crate::network_packet::NetworkPacket;

#[derive(Debug, Clone, Copy)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}

pub struct Connection {
    closed: bool,
    state: ConnectionState,
    stream: TcpStream,
}

impl Connection {
    fn new(stream: TcpStream) -> Self {
        Connection {
            closed: false,
            state: ConnectionState::Handshaking,
            stream,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    pub fn process<'a>(&'a mut self) {
        if let Some(network_packet) = self.get_packet() {
            info!("Packet:\n    state:{:?}\n    pid:{}", self.state, network_packet.id());
            let game_packet = crate::game_packet::get_game_packet::<'a>(self.state, network_packet.id(), network_packet.data_get());
            game_packet.log();
            game_packet.update_connection(self as *mut Connection);
            game_packet.respond(self.send_data_callback());
        }
    }

    pub fn change_state(&mut self, new_state: ConnectionState) {
        self.state = new_state;
    }

    fn send_data_callback<'a>(&'a mut self) -> Box<dyn FnMut(i32, Vec<u8>) + 'a> {
        Box::new(|packet_id: i32, data: Vec<u8>| {
            let packet_id_varint = types::VarInt::new(packet_id).write();
            let data_length = types::VarInt::new((packet_id_varint.len() + data.len()) as i32).write();
            self.stream.write_all(&data_length).unwrap();
            self.stream.write_all(&packet_id_varint).unwrap();
            self.stream.write_all(&data).unwrap();
            // TODO: handle errors
        })
    }

    fn get_packet(&mut self) -> Option<NetworkPacket> {
        let mut packet_length_buffer = [0u8; 5];
        let (packet_length, encoding_length) = match self.stream.peek(&mut packet_length_buffer) {
            Ok(read_bytes) => {
                if read_bytes == 0 {
                    info!("Connection from {} closed", self.stream.peer_addr().unwrap());
                    self.closed = true;
                    return None;
                } else {
                    types::VarInt::read(&packet_length_buffer)
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                return None; // Don't worry, we'll get 'em next iteration
            }
            Err(e) => panic!("encountered IO error: {e}"),
        };

        let mut packet_buffer = vec![0u8; (packet_length as usize) + encoding_length];
        let packet = match self.stream.read(&mut packet_buffer) {
            Ok(_) => {
                NetworkPacket::new(&packet_buffer)
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                return None; // Don't worry, we'll SURELY get 'em next iteration (i don't think we can actually get in here)
            }
            Err(e) => panic!("encountered IO error: {e}"),
        };
        trace!("created network connection");
        return Some(packet);
    }
}

pub struct ConnectionManager {
    pub conns: Vec<Connection>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        ConnectionManager {
            conns: Vec::new(),
        }
    }

    pub fn handle_connection(&mut self, stream: TcpStream) {
        info!("Got connection from {}", stream.peer_addr().unwrap());
        let mut peek_buffer = [0; 3];
        loop { // wait until we can read the whole buffer
            match stream.peek(&mut peek_buffer) {
                Ok(_) => {},
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP
                    // wait_for_fd();
                    continue;
                },
                Err(e) => panic!("encountered IO error: {e}"),
            };
            break;
        };

        static LEGACY_PING_HEADER: &[u8] = &[0xFE, 0x01, 0xFA];
        if peek_buffer.starts_with(LEGACY_PING_HEADER) {
            debug!("Recieved legacy ping from {}. Dropping connection", stream.peer_addr().unwrap());
            drop(stream);
            return;
        }

        self.conns.push(
            Connection::new(
                stream,
            )
        );
    }
}
