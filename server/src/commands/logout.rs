use crate::commands::ICommand::Command;
use crate::log_server;
use crate::{Client::Client, Server::Server};

pub struct logout_cmd {}

impl Command for logout_cmd {
    fn execute(&mut self, args: Vec<&str>, server: &mut Server, client: &mut Client) {
        if !args.is_empty() {
            client.add_data("Usage: /logout\n".into());
            client.send_data();
            return;
        }
        if client.name.is_empty() {
            client.add_data("ERROR: not logged in\n".into());
            client.send_data();
            return;
        }

        server.disconnect_user(client.id.as_str());
        let _ = log_server::event_user_logged_out(client.id.as_str());
        client.name.clear();
        client.selected_team = None;
        client.selected_channel = None;
        client.selected_thread = None;
        client.add_data("OK: logged out\n".into());
        client.send_data();
    }

    fn get_cmd(&self) -> String {
        "/logout".into()
    }
}
