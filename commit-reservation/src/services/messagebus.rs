use std::{collections::VecDeque, any::Any};
use crate::domain::event::Event;
use crate::services::handlers::{command_handler::CommandHandler, event_handler::EventHandler};

#[allow(dead_code)]
pub struct MessageBus{
    message_queue:VecDeque<Box<dyn Any>>
}

impl MessageBus{
    pub fn new() -> Self{
        Self{
            message_queue: VecDeque::new()
        }
    }
    #[allow(non_snake_case)]
    pub fn handle (
        &mut self,
        message: Box<dyn Any>
    ){
        self.message_queue.push_back(message);
        while let Some(msg) = self.message_queue.pop_front(){
            if msg.is::<Event>(){
                let eventHandler = EventHandler;
                eventHandler.execute(msg);
            }
            else{
                let commandHandler = CommandHandler;
                commandHandler.execute(msg);
            }
        }
    }
}