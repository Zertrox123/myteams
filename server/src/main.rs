mod Server;
mod Client;
mod Teams;
mod Channel;
mod ChatEntry;
mod log_server;

mod commands;
mod utils;

use commands::register_command;

use crate::commands::{create::create_cmd, help::help_cmd, join::join_cmd, r#use::use_cmd};

fn main() {
    register_command(Box::new(help_cmd   {}));
    register_command(Box::new(use_cmd    {}));
    register_command(Box::new(join_cmd   {}));
    register_command(Box::new(create_cmd {}));
    utils::get_fd_list();
    let mut srv = Server::Server::new("0.0.0.0:1337");


    srv.load();
    //println!("{:#?}", srv.as_slice());
    srv.run();
}
