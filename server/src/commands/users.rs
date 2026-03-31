use crate::commands::ICommand::Command;
use crate::{Client::Client, Server::Server};

pub struct users_cmd {}

impl Command for users_cmd {
    fn execute(&mut self, args: Vec<&str>, server: &mut Server, client: &mut Client) {
        if !args.is_empty() {
            client.add_data("Usage: /users\n".into());
            client.send_data();
            return;
        }

        let mut out = String::from("USERS:\n");
        for user in server.get_users() {
            out.push_str(format!("{} | {} | connected={}\n", user.id, user.name, user.connected).as_str());
        }
        client.add_data(out);
        client.send_data();
    }

    fn get_cmd(&self) -> String {
        "/users".into()
    }
}
