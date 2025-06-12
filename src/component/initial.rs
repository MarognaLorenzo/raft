use crate::component::Candidate;

use super::{Component, Initial};
use std::sync::mpsc::{self, Receiver, Sender};
use std::collections::HashMap;

impl <MessageType>Component<Initial, MessageType>{
    pub fn new(
        name: usize, 
        total_elements: usize, 
        rx: Receiver<MessageType>,
        neighbours: HashMap<usize,Sender<MessageType>>,
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

    pub fn add_sender(&mut self, name: usize, sender: Sender<MessageType>){
        self.neighbours.insert(name, sender);
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
