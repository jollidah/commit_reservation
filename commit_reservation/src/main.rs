// use std::any::Any;

// use domain::{command::GitCommand, event::Event};
// use services::messagebus::MessageBus;

mod services;
mod domain;

fn main() {
    // let mut mb = MessageBus::new();
    // &mb.handle(Box::new(GitCommand::Pull) as Box<dyn Any>);
    // mb.handle(Box::new(Event::GitPullSuccess) as Box<dyn Any>);
}

#[cfg(test)]
mod test;