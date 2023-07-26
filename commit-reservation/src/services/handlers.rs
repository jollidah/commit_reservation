use std::{process::Command, clone, str};
use crate::domain::command::GitCommand;

#[derive(Clone)]
pub struct ServiceHandler;

pub trait Handler{
    fn execute(self, cmd: GitCommand) -> String; 
}

impl Handler for ServiceHandler{
    fn execute(self, cmd: GitCommand) -> String{
        let result = match cmd {
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
    }
}
