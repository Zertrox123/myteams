use crate::commands::ICommand::Command;
use crate::{Client::Client, Server::Server};

pub struct info_cmd {}

impl Command for info_cmd {
    fn execute(&mut self, _args: Vec<&str>, server: &mut Server, client: &mut Client) {
        if client.selected_team.is_none() {
            client.add_data(format!("USER: {} | {}\n", client.id, client.name));
            client.send_data();
            return;
        }

        if client.selected_channel.is_none() {
            let team_id = client.selected_team.clone().unwrap();
            for team in &server.teams {
                if team.id != team_id { continue; }
                client.add_data(format!("TEAM: {} | {} | {}\n", team.id, team.name, team.description));
                client.send_data();
                return;
            }
            client.add_data("TEAM NOT FOUND\n".into());
            client.send_data();
            return;
        }

        if client.selected_thread.is_none() {
            let team_id = client.selected_team.clone().unwrap();
            let channel_id = client.selected_channel.clone().unwrap();
            for team in &server.teams {
                if team.id != team_id { continue; }
                for channel in &team.channels {
                    if channel.id != channel_id { continue; }
                    client.add_data(format!("CHANNEL: {} | {} | {}\n", channel.id, channel.name, channel.description));
                    client.send_data();
                    return;
                }
            }
            client.add_data("CHANNEL NOT FOUND\n".into());
            client.send_data();
            return;
        }

        let team_id = client.selected_team.clone().unwrap();
        let channel_id = client.selected_channel.clone().unwrap();
        let thread_id = client.selected_thread.clone().unwrap();
        for team in &server.teams {
            if team.id != team_id { continue; }
            for channel in &team.channels {
                if channel.id != channel_id { continue; }
                for thread in &channel.threads {
                    if thread.id != thread_id { continue; }
                    client.add_data(format!("THREAD: {} | {} | {}\n", thread.id, thread.name, thread.message));
                    client.send_data();
                    return;
                }
            }
        }
        client.add_data("THREAD NOT FOUND\n".into());
        client.send_data();
    }

    fn get_cmd(&self) -> String {
        "/info".into()
    }
}
