use std::{process::Command, any::Any};
use crate::domain::command::{GitCommand, DBCommand};


#[derive(Clone)]
pub struct CommandHandler;

impl CommandHandler{
    #[allow(dead_code)]
    pub fn execute(self, cmd: Box<dyn Any>) -> String{
        if cmd.is::<GitCommand>(){
            let git_command = *cmd.downcast::<GitCommand>().unwrap();
            let result: std::process::Output = match git_command {
                GitCommand::Pull{ .. } => {
                    Command::new("git")
                    .arg("pull")
                    .output()
                    .expect("error while git pull")
                }
                
                GitCommand::Add{ .. } => {
                    Command::new("git")
                    .arg("add *")
                    .output()
                    .expect("error while git add")
                }
                
                GitCommand::Commit{message} => {
                    Command::new("git")
                    .arg(format!("commit -m {}", message))
                    .output()
                    .expect("error while git commit")
                }
                
                GitCommand::Push{ .. } => {
                    Command::new("git")
                    .arg("push")
                    .output()
                    .expect("error while git push")
                }

                GitCommand::GetCommitLog{ .. } => {
                    Command::new("git")
                    .args(["log", "-1"])
                    .output()
                    .expect("error while git log")
                }
            };
            String::from_utf8_lossy(&result.stdout).to_string()
        } else{
            let db_command = *cmd.downcast::<DBCommand>().unwrap();
            // TODO DBCommand Handle
            match db_command{
                DBCommand::Push{ .. } => {
                    println!{"DBCommand Push"}
                    }
                DBCommand::Pull{ .. } => {
                    println!{"DBCommand Pull"}
                    }
                DBCommand::Update{ .. } => {
                    println!{"DBCommand Push"}
                    }
            };
            "db command recepted".to_string()
        }
    }
}
