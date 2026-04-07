mod Channel;
mod ChatEntry;
mod Client;
mod Server;
mod Teams;
mod log_server;

mod commands;
mod utils;

use commands::register_command;

use crate::commands::{create::create_cmd, help::help_cmd, info::info_cmd, join::join_cmd, list::list_cmd, login::login_cmd, logout::logout_cmd, subscribe::subscribe_cmd, subscribed::subscribed_cmd, r#use::use_cmd, user::user_cmd, users::users_cmd};

fn main() {
    register_command(Box::new(help_cmd {}));
    register_command(Box::new(use_cmd {}));
    register_command(Box::new(join_cmd {}));
    register_command(Box::new(create_cmd {}));
    register_command(Box::new(subscribed_cmd {}));
    register_command(Box::new(login_cmd {}));
    register_command(Box::new(logout_cmd {}));
    register_command(Box::new(subscribe_cmd {}));
    register_command(Box::new(users_cmd {}));
    register_command(Box::new(user_cmd {}));
    register_command(Box::new(list_cmd {}));
    register_command(Box::new(info_cmd {}));
    utils::get_fd_list();
    let mut srv = Server::Server::new("0.0.0.0:1337");

    srv.load();
    //println!("{:#?}", srv.as_slice());
    srv.run();
}
