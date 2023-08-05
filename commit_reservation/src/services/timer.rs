use crate::services::handlers::time_handler::TimeHandler;
use std::thread::sleep;

#[warn(while_true)]      // TODO Add Exit Logic
pub fn run(){
    while true{
        let time_handler = TimeHandler::new();
        sleep(time_handler.get_interval());
        // TODO Check if it needs to commit

    }
}

