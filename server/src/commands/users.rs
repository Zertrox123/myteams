use crate::commands::ICommand::Command;
use crate::{Client::Client, Server::Server};

pub struct users_cmd {}

impl Command for users_cmd {
    fn execute(&mut self, _: Vec<&str>, server: &mut Server, client: &mut Client) {
        client.add_data("users:".into());
        client.add_data(client.id.clone());
        client.add_data(":".into());
        client.add_data(client.name.clone());
        client.add_data("\r\n".into());
        for user in server.get_clients() {
            client.add_data(user.get(0).unwrap().to_string());
            client.add_data(":".into());
            client.add_data(user.get(1).unwrap().to_string());
            client.add_data("\r\n".into());
        }
        client.send_data();
    }

    fn get_cmd(&self) -> String {
        "/users".into()
    }
}
