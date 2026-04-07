use crate::commands::ICommand::Command;
use crate::{Client::Client, Server::Server};

pub struct subscribed_cmd {}

impl Command for subscribed_cmd {
    fn execute(&mut self, args: Vec<&str>, server: &mut Server, client: &mut Client) {
        if args.len() > 1 {
            client.add_data("Usage: /subscribed ?\"team_uuid\"\n".into());
            client.send_data();
            return;
        }

        if args.len() == 1 {
            let team_id = args[0];
            let users = server.get_subscribed_users(team_id);
            let mut out = String::from("SUBSCRIBED USERS:\n");
            for user in users {
                out.push_str(format!("{} | {}\n", user.id, user.name).as_str());
            }
            client.add_data(out);
            client.send_data();
            return;
        }

        let mut out = String::from("SUBSCRIPTIONS:\n");
        for team_id in client.get_subscribed_teams() {
            out.push_str(format!("{}\n", team_id).as_str());
        }
        client.add_data(out);
        client.send_data();
    }

    fn get_cmd(&self) -> String {
        "/subscribed".into()
    }
}
