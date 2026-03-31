use crate::commands::ICommand::Command;
use crate::{Client::Client, Server::Server};

pub struct messages_cmd {}

impl Command for messages_cmd {
    fn execute(&mut self, args: Vec<&str>, server: &mut Server, client: &mut Client) {
        if args.len() != 1 {
            client.add_data("Usage: /messages \"user_uuid\"\n".into());
            client.send_data();
            return;
        }
        if client.name.is_empty() {
            client.add_data("ERROR: not logged in\n".into());
            client.send_data();
            return;
        }
        let other = args[0];
        if server.find_user_by_id(other).is_none() {
            client.add_data("ERROR: unknown user\n".into());
            client.send_data();
            return;
        }

        let mut out = String::from("MESSAGES:\n");
        for msg in server.get_private_messages_between(client.id.as_str(), other) {
            out.push_str(format!("{} -> {} | {}\n", msg.from_uuid, msg.to_uuid, msg.body).as_str());
        }
        client.add_data(out);
        client.send_data();
    }

    fn get_cmd(&self) -> String {
        "/messages".into()
    }
}
