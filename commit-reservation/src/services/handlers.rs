use std::process::Command;
use crate::domain::command::GitCommand;

pub struct ServiceHandler;

pub trait Handler{
    fn execute(self, cmd: GitCommand); // TODO add return type;
}

impl Handler for ServiceHandler{
    fn execute(self, cmd: GitCommand){
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
        };
        println!{"{}", String::from_utf8_lossy(&result.stdout)};
    }
}
