use crate::commands::ICommand::Command;
use crate::log_server::event_user_logged_in;
use crate::{Client::Client, Server::Server};

pub struct login_cmd {}

impl Command for login_cmd {
    fn execute(&mut self, args: Vec<&str>, server: &mut Server, client: &mut Client) {
        if args.len() > 1 {
            client.add_data("Usage: /login \"username\"\n".into());
            client.send_data();
            return;
        }

        if args.len() == 1 {
            client.name = args.get(0).unwrap().to_string();
            event_user_logged_in(&client.id);
            client.add_data("OK".into());
            client.send_data();
            return;
        }
    }

    fn get_cmd(&self) -> String {
        "/login".into()
    }
}
