use crate::component::consensus_info::ConsensusInfo;
use crate::component::server_settings::ServerSettings;
use crate::component::{message::ServerMessage, order::Order};
use crate::component::{Follower, ServerT};

use super::{Initial, Server};
use crossbeam::channel::*;
use std::collections::HashMap;

impl Server<Initial> {
    pub fn new(
        name: usize,
        total_elements: usize,
        command_rx: Receiver<Order>,
        network_rx: Receiver<ServerMessage>,
        network_tx: Sender<ServerMessage>,
        neighbours: HashMap<usize, Sender<ServerMessage>>,
    ) -> Self {
        Server {
            _state: std::marker::PhantomData,
            name,
            order_rx: command_rx,
            message_rx: network_rx,
            self_transmitter: network_tx,
            neighbours,
            info: ConsensusInfo::new(),
            settings: ServerSettings::new(total_elements),
        }
    }

    pub fn add_sender(&mut self, name: usize, sender: Sender<ServerMessage>) {
        self.neighbours.insert(name, sender);
    }

    pub fn completed(self) -> Server<Follower> {
        let component = Server {
            _state: std::marker::PhantomData,
            name: self.name,
            order_rx: self.order_rx,
            message_rx: self.message_rx,
            self_transmitter: self.self_transmitter,
            neighbours: self.neighbours,
            info: self.info,
            settings: self.settings,
        };
        // component.candidate();
        return component;
    }
}

impl ServerT for Server<Initial> {
    fn handle_order(self: Box<Self>, _order: Order) -> (bool, Box<dyn ServerT>) {
        (true, Box::new(*self))
    }
    fn handle_server_message(
        self: Box<Self>,
        _message: ServerMessage,
    ) -> Box<dyn ServerT> {
        Box::new(*self)
    }
}
