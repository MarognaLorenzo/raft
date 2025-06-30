use crate::server::component::ServerComponents;
use crate::server::consensus_info::ConsensusInfo;
use crate::server::server_settings::ServerSettings;
use crate::server::{message::ServerMessage, order::Order};
use crate::server::{Follower, ServerT};

use super::{Initial, Server};
use crossbeam::channel::*;
use std::collections::{HashMap, VecDeque};

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
            info: ConsensusInfo::new(),
            settings: ServerSettings::new(total_elements),
            components: ServerComponents::new(
                command_rx,
                network_rx,
                network_tx,
                neighbours,
                VecDeque::new()
            ),
        }
    }

    pub fn add_sender(&mut self, name: usize, sender: Sender<ServerMessage>) {
        self.components.neighbours.insert(name, sender);
    }

    pub fn completed(self) -> Server<Follower> {
        let component = Server {
            _state: std::marker::PhantomData,
            name: self.name,
            info: self.info,
            settings: self.settings,
            components: self.components,
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
