use postgres::{Client, Error, NoTls};

// use std::any::Any;

// use domain::{command::GitCommand, event::Event};
// use services::messagebus::MessageBus;
static USER: &str = "jollidah";
static PASSWORD: &str = "abc123";
static PORT: &str = "5433";

mod services;
mod domain;

fn main() {
    let mut client = Client::connect(
        &format!(r"postgresql://{}:{}@localhost:{}/postgres", USER, PASSWORD, PORT),
        NoTls
    ).unwrap();
    

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS app_user (
            id                  SERIAL PRIMARY KEY,
            username            VARCHAR UNIQUE NOT NULL,
            password            VARCHAR NOT NULL,
            data                VARCHAR UNIQUE NOT NULL
        )
        ",
    ).unwrap();
    // let mut mb = MessageBus::new();
    // &mb.handle(Box::new(GitCommand::Pull) as Box<dyn Any>);
    // mb.handle(Box::new(Event::GitPullSuccess) as Box<dyn Any>);
}

#[cfg(test)]
mod test;