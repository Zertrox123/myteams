use crate::commands::ICommand::Command;
use crate::{Client::Client, Server::Server};

pub struct list_cmd {}

impl Command for list_cmd {
    fn execute(&mut self, _args: Vec<&str>, server: &mut Server, client: &mut Client) {
        if client.selected_team.is_none() {
            let mut out = String::from("TEAMS:\n");
            for team in &server.teams {
                out.push_str(&format!("{} | {} | {}\n", team.id, team.name, team.description));
            }
            client.add_data(out);
            client.send_data();
            return;
        }

        if client.selected_channel.is_none() {
            let team_id = client.selected_team.clone().unwrap();
            let mut out = String::from("CHANNELS:\n");
            for team in &server.teams {
                if team.id != team_id { continue; }
                for channel in &team.channels {
                    out.push_str(&format!("{} | {} | {}\n", channel.id, channel.name, channel.description));
                }
            }
            client.add_data(out);
            client.send_data();
            return;
        }

        if client.selected_thread.is_none() {
            let team_id = client.selected_team.clone().unwrap();
            let channel_id = client.selected_channel.clone().unwrap();
            let mut out = String::from("THREADS:\n");
            for team in &server.teams {
                if team.id != team_id { continue; }
                for channel in &team.channels {
                    if channel.id != channel_id { continue; }
                    for thread in &channel.threads {
                        out.push_str(&format!("{} | {} | {}\n", thread.id, thread.name, thread.message));
                    }
                }
            }
            client.add_data(out);
            client.send_data();
            return;
        }

        let team_id = client.selected_team.clone().unwrap();
        let channel_id = client.selected_channel.clone().unwrap();
        let thread_id = client.selected_thread.clone().unwrap();
        let mut out = String::from("REPLIES:\n");
        for team in &server.teams {
            if team.id != team_id { continue; }
            for channel in &team.channels {
                if channel.id != channel_id { continue; }
                for thread in &channel.threads {
                    if thread.id != thread_id { continue; }
                    for reply in &thread.reply {
                        out.push_str(&format!("{} | {} | {}\n", reply.id, reply.auth_name, reply.content));
                    }
                }
            }
        }
        client.add_data(out);
        client.send_data();
    }

    fn get_cmd(&self) -> String {
        "/list".into()
    }
}
