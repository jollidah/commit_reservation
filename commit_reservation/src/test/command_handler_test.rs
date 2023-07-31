#[cfg(test)]
pub mod command_handler_test{
    use std::fs;

    use crate::domain::command::GitCommand;
    use crate::test::helper::functions::make_test_file;
    use crate::services::handlers::command_handler::CommandHandler;


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
        let binding = command_handler_tester.execute(Box::new(GitCommand::Commit { message: (String::from("test commit"))}));
        let mut output = binding.split("\n");
        println!("{:?}", output.next());
        if output.next().unwrap() == " 1 file changed, 0 insertions(+), 0 deletions(-)"{
            return true;
        }
        false
    }

    pub fn git_push() -> bool{
        let command_handler_tester: CommandHandler = CommandHandler;
        command_handler_tester.execute(Box::new(GitCommand::Push));
        
        let command_handler_tester: CommandHandler = CommandHandler;
        let binding = command_handler_tester.execute(Box::new(GitCommand::Show));
        let mut output = binding.split("\n");
        for _ in 0..4{
            println!("{:?}", output.next());
        }
        if output.next().unwrap() == "    \"test commit\""{
            let command_handler_tester: CommandHandler = CommandHandler;
            command_handler_tester.execute(Box::new(GitCommand::ResetCommit));
            fs::remove_file("test.txt").unwrap();
            return true;
        }
        false
    }
}