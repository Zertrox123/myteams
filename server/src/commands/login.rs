use crate::commands::ICommand::Command;
use crate::log_server;
use crate::{Client::Client, Server::Server};

pub struct login_cmd {}

impl Command for login_cmd {
    fn execute(&mut self, args: Vec<&str>, server: &mut Server, client: &mut Client) {
        if args.len() != 1 {
            client.add_data("Usage: /login \"user_name\"\n".into());
            client.send_data();
            return;
        }

        let name = args[0];
        let user = server.add_or_connect_user(name);
        if client.id != user.id {
            client.id = user.id.clone();
        }
        client.name = user.name.clone();
        let _ = log_server::event_user_logged_in(client.id.as_str());
        client.add_data(format!("OK: logged in as {}\n", client.name));
        client.send_data();
    }

    fn get_cmd(&self) -> String {
        "/login".into()
    }
}
