use crate::component::consensus_info::ConsensusInfo;
use crate::component::{message::ServerMessage, order::Order};
use crate::component::{Candidate, Follower, ServerT};

use super::{Initial, Server};
use crossbeam::channel::*;
use std::collections::HashMap;

impl Server<Initial> {
    pub fn new(
        name: usize,
        total_elements: usize,
        command_rx: Receiver<Order>,
        network_rx: Receiver<ServerMessage>,
        neighbours: HashMap<usize, Sender<ServerMessage>>,
    ) -> Self {
        Server {
            _state: std::marker::PhantomData,
            name,
            total_elements,
            order_rx: command_rx,
            message_rx: network_rx,
            neighbours,
            info: ConsensusInfo::new(),
        }
    }

    pub fn add_sender(&mut self, name: usize, sender: Sender<ServerMessage>) {
        self.neighbours.insert(name, sender);
    }

    pub fn completed(self) -> Server<Follower> {
        let component = Server {
            _state: std::marker::PhantomData,
            name: self.name,
            total_elements: self.total_elements,
            order_rx: self.order_rx,
            message_rx: self.message_rx,
            neighbours: self.neighbours,
            info: self.info,
        };
        // component.candidate();
        return component;
    }
}

impl ServerT for Server<Initial> {
    fn handle_order(self: Box<Self>, order: Order) -> (bool, Box<dyn ServerT>) {
        (true, Box::new(*self))
    }
    fn handle_server_message(
        self: Box<Self>,
        message: super::message::ServerMessage,
    ) -> Box<dyn ServerT> {
        Box::new(*self)
    }
}
