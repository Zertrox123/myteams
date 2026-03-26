use crate::Teams::Team;
use crate::commands::ICommand::Command;
use crate::{Client::Client, Server::Server};

pub struct create_cmd{}

impl Command for create_cmd {
    fn execute(&mut self, args: Vec<&str>, server: &mut Server, client: &mut Client) {
        if client.selected_team.is_some() || client.selected_thread.is_some() || client.selected_channel.is_some() {
            
        } else if args.len() == 2 {
            let team_name        = args.get(0).unwrap().to_string();
            let team_description = args.get(1).unwrap().to_string();
            // TODO: being to make it work like tha t.do_that(a).and_that(b)
            let mut t = Team::new();
            t.set_name(team_name);
            t.set_description(team_description);
            server.add_team(t.clone());
        }

        if client.selected_team.is_none() {
            let team_name = args.get(0).unwrap();
        }
    }

    fn get_cmd(&self) -> String{
        "/create".into()
    }
}
