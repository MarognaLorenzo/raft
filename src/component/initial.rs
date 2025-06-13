use crate::component::message::Message;
use crate::component::Candidate;

use super::{Component, Initial};
use std::sync::mpsc::{self, Receiver, Sender};
use std::collections::HashMap;

impl Component<Initial>{
    pub fn new(
        name: usize, 
        total_elements: usize, 
        rx: Receiver<Message>,
        neighbours: HashMap<usize,Sender<Message>>,
    ) -> Self {
        Component{
            _state: std::marker::PhantomData,
            name,
            log : vec![],
            total_elements,
            rx,
            neighbours,
            term: 0,
            voted_for: None,
        }
    }

    pub fn add_sender(&mut self, name: usize, sender: Sender<Message>){
        self.neighbours.insert(name, sender);
    }

    pub fn activate(self) -> Component<Candidate>{
        let component = Component {
        _state: std::marker::PhantomData,
        log: self.log,
        name: self.name,
        total_elements: self.total_elements,
        rx: self.rx,
        neighbours: self.neighbours,
        term: self.term,
        voted_for: self.voted_for,
        };
        component.candidate();
        return component
    }

}
