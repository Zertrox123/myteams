use std::{net::TcpListener, thread, time::Duration};
use crate::{Client, Teams::Team, commands::execute};

#[derive(Debug)]
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

    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<Self>(),
            )
        }
    }

    fn as_struct(bytes: &[u8]) -> &Self {
        unsafe { &*(bytes.as_ptr() as *const Self) }
    }

    pub fn add_team(&mut self, t: Team){
        self.teams.push(t);
    }

    pub fn does_team_exist(&self, id: &String) -> Option<Team>
    {
        for team in self.teams.iter() {
            if &team.get_id() == id {
                return Some(team.clone());
            }
        }
        None
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
            let client = self.clients.get_mut(client_id).unwrap();
            client.recv_data();

            match client.get_input() {
                Ok(cmd) => {
                    if cmd == "DUMP\n" {
                        println!("{:#?}", self);
                    }
                    if !cmd.is_empty() {
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
