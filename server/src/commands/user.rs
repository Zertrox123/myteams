use crate::commands::ICommand::Command;
use crate::{Client::Client, Server::Server};

pub struct user_cmd {}

impl Command for user_cmd {
    fn execute(&mut self, args: Vec<&str>, server: &mut Server, client: &mut Client) {
        if args.len() != 1 {
            client.add_data("Usage: /user \"user_uuid\"\n".into());
            client.send_data();
            return;
        }
        let user_id = args[0];
        for user in &server.clients {
            if user.id == user_id {
                client.add_data(format!("{} | {}\n", user.id, user.name));
                client.send_data();
                return;
            }
        }
        client.add_data("USER NOT FOUND\n".into());
        client.send_data();
    }

    fn get_cmd(&self) -> String {
        "/user".into()
    }
}
