pub mod ICommand;
pub mod create;
pub mod help;
pub mod join;
pub mod r#use;

use std::sync::{Mutex, OnceLock};

use crate::{Client::Client, Server::Server};

pub static COMMANDS: OnceLock<Mutex<Vec<Box<dyn ICommand::Command + Send>>>> = OnceLock::new();

pub fn register_command(cmd: Box<dyn ICommand::Command + Send>) {
    COMMANDS
        .get_or_init(|| Mutex::new(vec![]))
        .lock()
        .unwrap()
        .push(cmd);
}

pub fn execute(mut cmd: String, server: &mut Server, client: &mut Client) {
    cmd.pop();
    let mut args: Vec<&str> = cmd.split(' ').collect();
    let command = args.remove(0);

    println!("CMD: {} args: {:#?}", command, args);

    let mut cmds = COMMANDS.get().unwrap().lock().unwrap();
    for cmd in cmds.iter_mut() {
        if cmd.get_cmd() == command {
            cmd.execute(args.clone(), server, client);
        }
    }
}
