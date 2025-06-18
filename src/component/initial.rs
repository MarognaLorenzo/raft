use crate::component::{message::ComponentMessage, order::Order};
use crate::component::Candidate;

use crossbeam::channel::*;
use super::{Component, Initial};
use std::collections::HashMap;

impl Component<Initial>{
    pub fn new(
        name: usize, 
        total_elements: usize, 
        command_rx: Receiver<Order>,
        network_rx: Receiver<ComponentMessage>,
        neighbours: HashMap<usize,Sender<ComponentMessage>>,
    ) -> Self {
        Component{
            _state: std::marker::PhantomData,
            name,
            log : vec![],
            total_elements,
            command_rx,
            network_rx,
            neighbours,
            term: 0,
            voted_for: None,
        }
    }

    pub fn add_sender(&mut self, name: usize, sender: Sender<ComponentMessage>){
        self.neighbours.insert(name, sender);
    }

    pub fn completed(self) -> Component<Candidate>{
        let component = Component {
        _state: std::marker::PhantomData,
        log: self.log,
        name: self.name,
        total_elements: self.total_elements,
        command_rx: self.command_rx,
        network_rx: self.network_rx,
        neighbours: self.neighbours,
        term: self.term,
        voted_for: self.voted_for,
        };
        // component.candidate();
        return component
    }

}
