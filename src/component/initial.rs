use crate::component::{message::ServerMessage, order::Order};
use crate::component::{Candidate, ServerT};

use crossbeam::channel::*;
use super::{Server, Initial};
use std::collections::HashMap;

impl Server<Initial>{
    pub fn new(
        name: usize, 
        total_elements: usize, 
        command_rx: Receiver<Order>,
        network_rx: Receiver<ServerMessage>,
        neighbours: HashMap<usize,Sender<ServerMessage>>,
    ) -> Self {
        Server{
            _state: std::marker::PhantomData,
            name,
            log : vec![],
            total_elements,
            order_rx: command_rx,
            message_rx: network_rx,
            neighbours,
            term: 0,
            voted_for: None,
        }
    }

    pub fn add_sender(&mut self, name: usize, sender: Sender<ServerMessage>){
        self.neighbours.insert(name, sender);
    }

    pub fn completed(self) -> Server<Candidate>{
        let component = Server {
        _state: std::marker::PhantomData,
        log: self.log,
        name: self.name,
        total_elements: self.total_elements,
        order_rx: self.order_rx,
        message_rx: self.message_rx,
        neighbours: self.neighbours,
        term: self.term,
        voted_for: self.voted_for,
        };
        // component.candidate();
        return component
    }

}


impl ServerT for Server<Initial>{
    fn handle_order(self: Box<Self>, order: Order) -> (bool, Box<dyn ServerT>) {
        (true, Box::new(*self)) 
    }
    fn handle_server_message(self: Box<Self>, message: super::message::ServerMessage) -> Box<dyn ServerT> {
        Box::new(*self)
    }
    /* fn handle_order(&self, order: super::order::Order) -> bool {
        match order {
            Order::SendInfo { info } => {
                println!("Received: {}", info);
                true
            }
        }
    } */
}
