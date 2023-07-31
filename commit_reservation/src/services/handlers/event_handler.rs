use std::any::Any;

use crate::domain::event::Event;

#[allow(dead_code)]
pub struct EventHandler;

impl EventHandler{ // TODO deal EventHandler
    #[allow(dead_code)]
    pub fn execute(self, event: Box<dyn Any>){
        let event = *event.downcast::<Event>().unwrap();
        match event{
            Event::GitPullSuccess{ .. } => {
                println!("Git Pull Success", );}
            Event::GitPullFailed{ .. } => {
                println!("Git Pull Failed", );}
            Event::GitAddSuccess{ .. } => {
                println!("Git Add Success", );}
            Event::GitAddFailed{ .. } => {
                println!("Git Add Failed", );}
            Event::GitCommitSuccess{ .. } => {
                println!("Git Commit Success", );}
            Event::GitCommitFailed{ .. } => {
                println!("Git Commit Failed", );}
            Event::GitGetCommitLogSuccess{ .. } => {
                println!("Git GetCommitLog Success", );}
            Event::GitGetCommitLogFailed{ .. } => {
                println!("Git GetCommitLog Failed", );}
        }
    }

}
