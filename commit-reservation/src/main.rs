use services::handlers::Handler;

use crate::{
    services::handlers::ServiceHandler,
    domain::command::GitCommand
};
// use std::process::{Command};
mod domain;
mod services;

fn main() {
    // let result = Command::new("git")
    // .arg("pull")
    // .output()
    // .expect("failed while git pull");

    // println!("{}", String::from_utf8_lossy(&result.stdout));
    let handler = ServiceHandler;
    let testCommand = GitCommand::Pull;
    handler.execute(testCommand);
}