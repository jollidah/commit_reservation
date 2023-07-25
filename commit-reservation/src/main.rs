use services::handlers::Handler;

use crate::{
    services::handlers::ServiceHandler,
    domain::command::GitCommand
};
mod domain;
mod services;

fn main() {
    let handler = ServiceHandler;
    let test_command = GitCommand::Pull;
    handler.execute(test_command);
}