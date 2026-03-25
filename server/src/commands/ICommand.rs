use crate::{Client::Client, Server::Server};

pub trait Command {
    fn execute(&mut self, args: Vec<&str>, server: &mut Server, client: &mut Client);
    fn get_cmd(&self) -> String;
}
