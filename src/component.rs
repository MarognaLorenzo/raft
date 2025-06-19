use std::{collections::HashMap, };
use std::fmt::Debug;
pub mod consensus_info;
pub mod order;
use order::Order;
pub mod message;
use message::ServerMessage;
use crossbeam::channel::*;

use crate::component::consensus_info::ConsensusInfo;

#[derive(Debug)]
pub struct Initial;
#[derive(Debug)]
pub struct Leader;
#[derive(Debug)]
pub struct Candidate{
    voting_received: usize,
}
#[derive(Debug)]
pub struct Follower{
    leader: usize,
}

pub trait ServerT: Debug{
    fn handle_server_message(self: Box<Self>, _message: ServerMessage) -> Box<dyn ServerT>{
        panic!("I should be implemented!")
    }

    fn handle_order(self: Box<Self>, _order: Order) -> (bool, Box<dyn ServerT>){
        panic!("I should be implemented!")
    }
}
pub trait StateT{}

impl StateT for Initial {}
impl StateT for Leader {}
impl StateT for Follower {}
impl StateT for Candidate {}

#[derive(Debug)]
pub struct Server<S: StateT> {
    _state: std::marker::PhantomData<S>,
    name: usize,
    total_elements: usize,
    order_rx: Receiver<Order>,
    message_rx: Receiver<ServerMessage>,
    pub neighbours: HashMap<usize, Sender<ServerMessage>>,
    info: ConsensusInfo,
}


mod initial;
mod candidate;
mod leader;
mod follower;
mod common;

