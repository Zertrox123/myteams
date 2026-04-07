use crate::commands::ICommand::Command;
use crate::log_server::event_user_logged_in;
use crate::{Client::Client, Server::Server};

pub struct logout_cmd {}

impl Command for logout_cmd {
    fn execute(&mut self, args: Vec<&str>, server: &mut Server, client: &mut Client) {
        client.set_dead();
    }

    fn get_cmd(&self) -> String {
        "/logout".into()
    }
}
