use crate::commands::ICommand::Command;
use crate::log_server::event_user_subscribed;
use crate::{Client::Client, Server::Server};

pub struct subscribe_cmd {}

impl Command for subscribe_cmd {
    fn execute(&mut self, args: Vec<&str>, server: &mut Server, client: &mut Client) {
        if args.len() > 1 {
            client.add_data("Usage: /subscribe ?\"team_uuid\"\n".into());
            client.send_data();
            return;
        }

        if args.len() == 1 {
            let team_id = args[0];
            if !server.does_team_exist(team_id).is_some() {
                client.add_data("TEAM ID DOES NOT EXIST".into());
                client.send_data();
                return;
            }
            client.subscribe_to_team(team_id);
            event_user_subscribed(team_id, &client.id);
            return;
        }
    }

    fn get_cmd(&self) -> String {
        "/subscribe".into()
    }
}
