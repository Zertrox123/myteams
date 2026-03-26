use crate::commands::ICommand::Command;
use crate::{Client::Client, Server::Server};

pub struct join_cmd {}

impl join_cmd {
    
}

impl Command for join_cmd {
    fn execute(&mut self, _args: Vec<&str>, _server: &mut Server, client: &mut Client) {
        client.add_data("yoo".into());
        client.send_data();
    }

    fn get_cmd(&self) -> String{
        "/join".into()
    }
}
