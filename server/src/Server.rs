use crate::{Channel, Client, Teams::Team, commands::execute};
use std::{fs, net::TcpListener, thread, time::Duration};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
    clients: Vec<Client::Client>,
    teams: Vec<Team>,
}

impl Server {
    pub fn new(addr: &str) -> Server {
        Server {
            listener: TcpListener::bind(addr).unwrap(),
            clients: Vec::new(),
            teams: Vec::new(),
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

    pub fn load(&mut self) {
        let base_path = "data";
        let server_str = fs::read_to_string(format!("{}/metadata", base_path)).unwrap();

        for team_entry in fs::read_dir(base_path).unwrap() {
            let team_path = team_entry.unwrap().path();
            if !team_path.is_dir() {
                continue;
            }
            let team_str = fs::read_to_string(format!("{}/metadata", team_path.display())).unwrap();
            let mut team = Team::from_string(team_str.lines().collect::<Vec<&str>>()).unwrap();

            for channel_entry in fs::read_dir(&team_path).unwrap() {
                let channel_path = channel_entry.unwrap().path();
                if !channel_path.is_dir() {
                    continue;
                }
                let channel_str =
                    fs::read_to_string(format!("{}/metadata", channel_path.display())).unwrap();
                let mut channel =
                    Channel::Channel::from_string(channel_str.lines().collect::<Vec<&str>>())
                        .unwrap();

                for thread_entry in fs::read_dir(&channel_path).unwrap() {
                    let thread_path = thread_entry.unwrap().path();
                    if !thread_path.is_dir() {
                        continue;
                    }
                    let thread_str =
                        fs::read_to_string(format!("{}/metadata", thread_path.display())).unwrap();
                    channel.threads.push(
                        Channel::Threads::from_string(thread_str.lines().collect::<Vec<&str>>())
                            .unwrap(),
                    );
                }

                team.channels.push(channel);
            }

            self.teams.push(team);
        }
    }
    pub fn save(&mut self) {
        let base_path = "data";
        let metadata_path = format!("{}/metadata", base_path);
        if fs::metadata(&base_path).is_err() {
            let _ = fs::create_dir_all(&base_path);
        }
        let _ = fs::write(metadata_path, self.to_string());
        for team in self.teams.iter_mut() {
            let team_path = format!("{}/{}", base_path, team.id);
            let metadata_path = format!("{}/metadata", team_path);
            if fs::metadata(&team_path).is_err() {
                let _ = fs::create_dir_all(&team_path);
            }
            let _ = fs::write(metadata_path, team.to_string());
            for channel in team.channels.iter_mut() {
                let channel_path = format!("{}/{}", team_path, channel.id);
                let metadata_path = format!("{}/metadata", channel_path);
                if fs::metadata(&channel_path).is_err() {
                    let _ = fs::create_dir_all(&channel_path);
                }
                let _ = fs::write(metadata_path, channel.to_string());
                for thread in channel.threads.iter_mut() {
                    let thread_path = format!("{}/{}", channel_path, thread.id);
                    let metadata_path = format!("{}/metadata", thread_path);
                    if fs::metadata(&thread_path).is_err() {
                        let _ = fs::create_dir_all(&thread_path);
                    }
                    let _ = fs::write(metadata_path, thread.to_string());
                }
            }
        }
    }

    fn as_struct(bytes: &[u8]) -> &Self {
        unsafe { &*(bytes.as_ptr() as *const Self) }
    }

    pub fn add_team(&mut self, t: Team) {
        self.teams.push(t);
    }

    pub fn does_team_exist(&self, id: &String) -> Option<Team> {
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
            match self.clients.get_mut(client_index) {
                Some(client) => {
                    if client.is_dead() {
                        self.clients.remove(client_index);
                        self.clients.retain(|c| c.is_dead());
                    }
                }
                None => {}
            }
        }
    }

    pub fn to_string(&self) -> String {
        format!("")
    }
}
