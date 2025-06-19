use std::{collections::HashMap, };

pub mod order;
use order::Order;
pub mod message;
use message::ServerMessage;
use crossbeam::channel::*;

pub struct Initial;
pub struct Leader;
pub struct Candidate{
    voting_received: usize,
}
pub struct Follower{
    leader: usize,
}

pub trait ServerT {
    fn handle_component_message(&self, message: ServerMessage) {
        panic!("I should be implemented!")
    }

    fn handle_order(&self, order: Order) -> bool{
        panic!("I should be implemented!")
    }
}
pub trait StateT{}

impl StateT for Initial {}
impl StateT for Leader {}
impl StateT for Follower {}
impl StateT for Candidate {}

pub struct Server<S: StateT> {
    _state: std::marker::PhantomData<S>,
    log: Vec<i32>,
    name: usize,
    total_elements: usize,
    order_rx: Receiver<Order>,
    message_rx: Receiver<ServerMessage>,
    pub neighbours: HashMap<usize, Sender<ServerMessage>>,
    term: usize,
    voted_for: Option<usize>,
}


mod initial;
mod candidate;
mod leader;
mod follower;
mod common;

