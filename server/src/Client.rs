use std::{
    io::{Read, Write},
    net::TcpStream,
    string::FromUtf8Error,
};

use crate::utils;

#[derive(Debug)]
pub struct Client {
    pub name: String,
    socket: TcpStream,
    teams: Vec<String>,
    input: Vec<u8>,
    output: Vec<u8>,
    dead: bool,
    pub id: String,
    pub selected_team: Option<String>,
    pub selected_channel: Option<String>,
    pub selected_thread: Option<String>,
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        Client {
            name: String::new(),
            selected_team: None,
            selected_channel: None,
            selected_thread: None,
            socket: stream,
            id: utils::generate_uuid(),
            teams: Vec::new(),
            input: Vec::new(),
            output: Vec::new(),
            dead: false,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.dead
    }

    pub fn recv_data(&mut self) {
        if self.dead {
            return;
        }
        let _ = self.socket.read_to_end(&mut self.input);

        let mut buf = [0u8; 1];
        self.dead = match self.socket.peek(&mut buf) {
            Ok(0) => true,
            Ok(_) => false,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => false,
            Err(_) => true,
        };
    }

    pub fn get_input(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.input.clone())
    }

    pub fn reset_input(&mut self) {
        self.input = Vec::new();
    }

    pub fn add_data(&mut self, message: String) {
        self.output.append(&mut message.as_bytes().to_vec());
    }
    pub fn unsubscribe_from_team(&mut self, team: &str) {
        self.teams.retain(|t| t != team);
    }

    pub fn send_data(&mut self) {
        if self.dead {
            return;
        }
        match self.socket.write(&self.output) {
            Ok(_) => {}
            Err(_e) => {
                self.dead = true;
                let _ = self.socket.shutdown(std::net::Shutdown::Both);
            }
        }
        self.output = Vec::new();
    }

    pub fn set_dead(&mut self) {
        self.dead = true;
    }
    
    pub fn get_subscribed_teams(&self) -> Vec<String> {
        self.teams.clone()
    }

    pub fn subscribe_to_team(&mut self, team: &str) {
        self.teams.push(team.to_string());
    }
}
