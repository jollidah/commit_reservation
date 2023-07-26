use services::handlers::Handler;

use crate::{
    services::handlers::ServiceHandler,
    domain::command::GitCommand
};
mod domain;
mod services;

fn main() {
    let handler = ServiceHandler;
    handler.clone().execute(GitCommand::Pull);
    handler.execute(GitCommand::GetCommitLog);
}