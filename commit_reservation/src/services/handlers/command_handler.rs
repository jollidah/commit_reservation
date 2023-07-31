use std::{process::Command, any::Any, fmt::format};
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
                
                GitCommand::Add{ path } => {
                    Command::new("git")
                    .args(["add", format!("{}", path).as_str()])
                    .output()
                    .expect("error while git add")
                }
                
                GitCommand::Commit{message} => {
                    Command::new("git")
                    .args(["commit", "-m", format!("\"{}\"", message).as_str()])
                    .output()
                    .expect("error while git commit")
                }
                
                GitCommand::Push{ .. } => {
                    Command::new("git")
                    .args(["push"])
                    .output()
                    .expect("error while git push")
                }

                GitCommand::GetCommitLog{ .. } => {
                    Command::new("git")
                    .args(["log", "-1"])
                    .output()
                    .expect("error while git log")
                }

                GitCommand::Status{ .. } => {
                    Command::new("git")
                    .args(["status"])
                    .output()
                    .expect("error while git log")
                }

                GitCommand::ResetCommit{ .. } => {
                    Command::new("git")
                    .args(["reset", "--hard", "HEAD~1"])
                    .output()
                    .expect("error while git reset");

                    Command::new("git")
                    .args(["push", "-f", "origin", "master"])
                    .output()
                    .expect("error while git reset")
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
