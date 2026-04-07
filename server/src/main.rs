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
use std::env;

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
    match env::args().nth(1) {
        Some(data) => {
            let is_num = data.parse::<usize>();
            if is_num.is_ok() {
                let port = is_num.unwrap();
                let mut srv = Server::Server::new(
                    format!("0.0.0.0:{}", port).as_str()
                );
                srv.load();
                srv.run();
                return;
            }
            println!("super help message that ill finish later");
        }
        None => {
            println!("super help message that ill finish later");
        }
    }
 
}
