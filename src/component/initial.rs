use crate::component::Candidate;

use super::{Component, Initial};
use std::sync::mpsc::{self, Receiver, Sender};

impl <MessageType>Component<Initial, MessageType>{
    pub fn new(
        name: i32, 
        total_elements: i32, 
        rx: Receiver<MessageType>,
        neighbours: Vec<Sender<MessageType>>,
    ) -> Self {
        Component{
            _state: std::marker::PhantomData,
            name,
            log : vec![],
            total_elements,
            rx,
            neighbours,
        }
    }

    pub fn add_sender(&mut self, sender: Sender<MessageType>){
        self.neighbours.push(sender);
    }

    pub fn activate(self) -> Component<Candidate, MessageType>{
        Component {
        _state: std::marker::PhantomData,
        log: self.log,
        name: self.name,
        total_elements: self.total_elements,
        rx: self.rx,
        neighbours: self.neighbours,
        }
    }

}
