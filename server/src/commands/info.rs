use crate::commands::ICommand::Command;
use crate::{Client::Client, Server::Server};

pub struct info_cmd {}

impl Command for info_cmd {
    fn execute(&mut self, args: Vec<&str>, server: &mut Server, client: &mut Client) {
        if !args.is_empty() {
            client.add_data("Usage: /info\n".into());
            client.send_data();
            return;
        }

        if client.selected_team.is_none() {
            client.add_data(format!("ME: {} | {}\n", client.id, client.name));
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
            client.add_data(format!("TEAM: {} | {} | {}\n", team.id, team.name, team.description));
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
            client.add_data(format!("CHANNEL: {} | {} | {}\n", channel.id, channel.name, channel.description));
            client.send_data();
            return;
        }

        let thread_id = client.selected_thread.clone().unwrap_or_default();
        let Some(thread) = server.get_thread(team_id.as_str(), channel_id.as_str(), thread_id.as_str()) else {
            client.add_data("ERROR: unknown thread\n".into());
            client.send_data();
            return;
        };
        client.add_data(format!("THREAD: {} | {} | {}\n", thread.id, thread.title, thread.message));
        client.send_data();
    }

    fn get_cmd(&self) -> String {
        "/info".into()
    }
}
