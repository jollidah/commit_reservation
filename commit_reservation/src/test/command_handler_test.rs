#[cfg(test)]
pub mod command_handler_test{
    use std::usize;

    use crate::domain::command::GitCommand;
    use crate::test::helper::functions::{make_test_file, rese};
    use crate::services::handlers::command_handler::CommandHandler;
    use super::*;

    pub fn git_pull() -> bool {
        let command_handler_tester = CommandHandler;
        command_handler_tester.execute(Box::new(GitCommand::Pull));
        let command_handler_tester = CommandHandler;
        match command_handler_tester.execute(Box::new(GitCommand::Pull)).as_str(){
            "Already up to date.\n" => true,
            _ => false
        }
    }

    pub fn git_add() -> bool{
        if make_test_file(){
            let command_handler_tester = CommandHandler;
            command_handler_tester.execute(Box::new(GitCommand::Add { path: String::from(".\\test.txt")}));
            let command_handler_tester = CommandHandler;
            let output = command_handler_tester.execute(Box::new(GitCommand::Status));
            // println!("{:?}", output);
            let output = output.split_whitespace().filter(|x| **x == String::from("new") || **x == String::from("file:") || **x == String::from("test.txt"));
            if output.count() == 3{
                return true;
            }
        }
        false
    }

    pub fn git_commit() -> bool{
        let command_handler_tester: CommandHandler = CommandHandler;
        let output = command_handler_tester.execute(Box::new(GitCommand::Commit { message: (String::from("test commit"))}));
        println!("{:?}", output);
        if output.split("\n").next().unwrap() == " 1 file changed, 0 insertions(+), 0 deletions(-)"{
            return true;
        }
        false
    }

    #[test]
    pub fn test_git_push() -> bool{
        let mut test_result : bool = false;
        let command_handler_tester: CommandHandler = CommandHandler;
        let mut output = command_handler_tester.execute(Box::new(GitCommand::Commit { message: (String::from("test commit"))})).split("\n");
        for _ in 0..5{
            output.next();
        }
        if output.next().unwrap() == "remote: Resolving deltas: 100% (1/1), completed with 1 local object."{
            test_result = true;
        }
        let command_handler_tester: CommandHandler = CommandHandler;
        command_handler_tester.execute(Box::new(GitCommand::ResetCommit));
        test_result
    }
}