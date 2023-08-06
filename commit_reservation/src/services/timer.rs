use crate::{services::handlers::{time_handler::TimeHandler, command_handler::CommandHandler}, domain::command::GitCommand};
use std::thread::sleep;

#[warn(while_true)]      // TODO Add Exit Logic
pub fn run(){
    while true{
        let time_handler1 = TimeHandler::new();
        let time_handler2 = TimeHandler::new();
        println!("Start Timer");
        sleep(time_handler1.get_interval());
        println!("Start Auto Commit at {:?}", time_handler2.get_checking_time());
        auto_commit();
        println!("Successed Auto Commit");
        break;
        // TODO Check if it needs to commit
        // if need_commit(){
        //     auto_commit();
        // }
    }
}

fn need_commit() -> bool{ // TODO Use MessageBus
        let command_handler_tester: CommandHandler = CommandHandler;
        let output = command_handler_tester.execute(Box::new(GitCommand::GetTodaysCommit));
        if output.is_empty(){
            return true
        }
        false
    }

fn auto_commit(){
    let command_handler_tester: CommandHandler = CommandHandler;
    let output = command_handler_tester.execute(Box::new(GitCommand::Pull));
    let command_handler_tester: CommandHandler = CommandHandler;
    let output = command_handler_tester.execute(Box::new(GitCommand::Add { path: "*".to_string() }));
    let command_handler_tester: CommandHandler = CommandHandler;
    let output = command_handler_tester.execute(Box::new(GitCommand::Commit { message: "Auto Commit Test".to_string() }));
    let command_handler_tester: CommandHandler = CommandHandler;
    let output = command_handler_tester.execute(Box::new(GitCommand::Push));
}