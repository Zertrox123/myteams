use crate::Teams::Team;
use crate::commands::ICommand::Command;
use crate::{Client::Client, Server::Server};

pub struct use_cmd {}

impl use_cmd {}

impl Command for use_cmd {
    fn execute(&mut self, args: Vec<&str>, server: &mut Server, client: &mut Client) {
        if client.selected_team.is_none() {
            let team_id = args.get(0).unwrap().to_string();
            if server.does_team_exist(&team_id).is_some() {
                client.selected_team = Some(team_id);
            }
            return;
        }
        if client.selected_channel.is_none() {
            let team_id = args.get(0).unwrap().to_string();
            client.selected_team = Some(team_id);
            return;
        }
    }

    fn get_cmd(&self) -> String {
        "/use".into()
    }
}
