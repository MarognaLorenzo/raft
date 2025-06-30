use std::collections::{HashMap, VecDeque};

use crossbeam::channel::{Receiver, Sender};

use crate::server::{message::ServerMessage, order::Order};

#[derive(Debug, Clone)]
pub struct ServerComponents{
    pub order_rx: Receiver<Order>,
    pub message_rx: Receiver<ServerMessage>,
    pub self_transmitter: Sender<ServerMessage>,
    pub neighbours: HashMap<usize, Sender<ServerMessage>>,
    pub message_queue: VecDeque<String>,
}


impl ServerComponents {
    pub fn new(
        order_rx: Receiver<Order>,
        message_rx: Receiver<ServerMessage>,
        self_transmitter: Sender<ServerMessage>,
        neighbours: HashMap<usize, Sender<ServerMessage>>,
        message_queue: VecDeque<String>,
    ) -> ServerComponents {
        ServerComponents { 
            order_rx,
            message_rx,
            self_transmitter,
            neighbours,
            message_queue,
        }
    }
}
