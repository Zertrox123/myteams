use crate::commands::ICommand::Command;
use crate::log_server;
use crate::{Client::Client, Server::Server};

pub struct send_cmd {}

impl Command for send_cmd {
    fn execute(&mut self, args: Vec<&str>, server: &mut Server, client: &mut Client) {
        if args.len() != 2 {
            client.add_data("Usage: /send \"user_uuid\" \"message_body\"\n".into());
            client.send_data();
            return;
        }
        if client.name.is_empty() {
            client.add_data("ERROR: not logged in\n".into());
            client.send_data();
            return;
        }

        let receiver = args[0];
        let body = args[1];
        if server.find_user_by_id(receiver).is_none() {
            client.add_data("ERROR: unknown user\n".into());
            client.send_data();
            return;
        }

        server.add_private_message(client.id.as_str(), receiver, body);
        let _ = log_server::event_private_message_sent(client.id.as_str(), receiver, body);
        client.add_data("OK: message sent\n".into());
        client.send_data();
    }

    fn get_cmd(&self) -> String {
        "/send".into()
    }
}
