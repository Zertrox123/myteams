use crate::commands::ICommand::Command;
use crate::log_server;
use crate::{Client::Client, Server::Server};

pub struct subscribe_cmd {}

impl Command for subscribe_cmd {
    fn execute(&mut self, args: Vec<&str>, server: &mut Server, client: &mut Client) {
        if args.len() != 1 {
            client.add_data("Usage: /subscribe \"team_uuid\"\n".into());
            client.send_data();
            return;
        }
        let team_id = args[0].to_string();
        if server.does_team_exist(&team_id).is_none() {
            client.add_data("ERROR: unknown team\n".into());
            client.send_data();
            return;
        }

        client.subscribe_team(team_id.clone());
        let _ = log_server::event_user_subscribed(team_id.as_str(), client.id.as_str());
        client.add_data("OK: subscribed\n".into());
        client.send_data();
    }

    fn get_cmd(&self) -> String {
        "/subscribe".into()
    }
}
