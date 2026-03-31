use crate::commands::ICommand::Command;
use crate::{Client::Client, Server::Server};

pub struct list_cmd {}

impl Command for list_cmd {
    fn execute(&mut self, args: Vec<&str>, server: &mut Server, client: &mut Client) {
        if !args.is_empty() {
            client.add_data("Usage: /list\n".into());
            client.send_data();
            return;
        }

        if client.selected_team.is_none() {
            let mut out = String::from("TEAMS:\n");
            for team in server.get_teams() {
                out.push_str(format!("{} | {} | {}\n", team.id, team.name, team.description).as_str());
            }
            client.add_data(out);
            client.send_data();
            return;
        }

        let team_id = client.selected_team.clone().unwrap_or_default();
        if client.selected_channel.is_none() {
            let Some(team) = server.get_team(team_id.as_str()) else {
                client.add_data("ERROR: unknown team\n".into());
                client.send_data();
                return;
            };
            let mut out = String::from("CHANNELS:\n");
            for ch in &team.channels {
                out.push_str(format!("{} | {} | {}\n", ch.id, ch.name, ch.description).as_str());
            }
            client.add_data(out);
            client.send_data();
            return;
        }

        let channel_id = client.selected_channel.clone().unwrap_or_default();
        if client.selected_thread.is_none() {
            let Some(channel) = server.get_channel(team_id.as_str(), channel_id.as_str()) else {
                client.add_data("ERROR: unknown channel\n".into());
                client.send_data();
                return;
            };
            let mut out = String::from("THREADS:\n");
            for th in &channel.threads {
                out.push_str(format!("{} | {} | {}\n", th.id, th.title, th.message).as_str());
            }
            client.add_data(out);
            client.send_data();
            return;
        }

        let thread_id = client.selected_thread.clone().unwrap_or_default();
        let Some(thread) = server.get_thread(team_id.as_str(), channel_id.as_str(), thread_id.as_str()) else {
            client.add_data("ERROR: unknown thread\n".into());
            client.send_data();
            return;
        };
        let mut out = String::from("REPLIES:\n");
        for reply in &thread.replies {
            out.push_str(format!("{} | {} | {}\n", reply.id, reply.auth_name, reply.content).as_str());
        }
        client.add_data(out);
        client.send_data();
    }

    fn get_cmd(&self) -> String {
        "/list".into()
    }
}
