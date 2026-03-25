use std::{net::{self, TcpListener}, os::linux::net::TcpStreamExt, thread, time::Duration};
use crate::{Client, Teams::Team, commands::execute};
use crate::utils;

pub struct Server {
    listener: TcpListener,
    clients: Vec<Client::Client>,
    teams: Vec<Team>,
}

impl Server {
    pub fn new(addr: &str) -> Server{
        Server { 
            listener: TcpListener::bind(addr).unwrap(),
            clients: Vec::new(),
            teams: Vec::new()
        }
    }

    pub fn run(&mut self) {
        let _ = self.listener.set_nonblocking(true);

        loop {

            //if self.clients.len() > 1 {
            if true {
                for stream in self.listener.incoming() {
                    if stream.is_err() {
                        thread::sleep(Duration::from_millis(50));
                        break;
                    }
                    let stream = stream.unwrap();
                    stream.set_ttl(1);
                    let _ = stream.set_nonblocking(true);
                    self.clients.push(Client::Client::new(stream));

                    println!("Connection established!");
                } 
            }

            for client in &mut self.clients {
                client.recv_data();
            }
            self.handle_commands();
        }
    }

    fn handle_commands(&mut self) {
        for client_id in 0..self.clients.len() {
            let mut client = self.clients.get_mut(client_id).unwrap();
            client.recv_data();
            match client.get_input() {
                Ok(cmd) => {
                    if cmd.len() > 0 {
                        let mut client = self.clients.remove(client_id);
                        execute(cmd, self, &mut client);
                        client.reset_input();
                        self.clients.push(client);
                    }
                }
                Err(_) => {
                    client.reset_input();
                }
            }
        }

        for client_index in 0..self.clients.len() {
            if let Some(client) = self.clients.get_mut(client_index) && client.is_dead(){
                    self.clients.remove(client_index);
                    self.clients.retain(|c| c.is_dead());
            }
        }
    }
}
