mod Server;
mod Client;
mod Teams;
mod Channel;
mod ChatEntry;

mod commands;
mod utils;

use commands::register_command;

use crate::{commands::{help::{self, help_cmd}, join::{self, join_cmd}}, utils::generate_uuid};

fn main() {
    register_command(Box::new(help_cmd {}));
    register_command(Box::new(join_cmd {} ));
    utils::get_fd_list();
    let mut srv = Server::Server::new("0.0.0.0:1337");

    srv.run();
}
