use std::collections::HashMap;
use std::fmt::{Debug, Display};
pub mod consensus_info;
pub mod order;
use order::Order;
pub mod message;
pub mod server_settings;
use server_settings::*;
use crossbeam::channel::*;
use message::ServerMessage;

use crate::component::consensus_info::ConsensusInfo;

#[derive(Debug)]
pub struct Initial;
#[derive(Debug)]
pub struct Leader;
#[derive(Debug)]
pub struct Candidate;
#[derive(Debug)]
pub struct Follower;

pub trait ServerT: Debug + Display {
    fn handle_server_message(self: Box<Self>, _message: ServerMessage) -> Box<dyn ServerT> {
        unimplemented!()
    }

    fn handle_order(self: Box<Self>, _order: Order) -> (bool, Box<dyn ServerT>) {
        unimplemented!()
    }
}
pub trait StateT {}

impl StateT for Initial {}
impl StateT for Leader {}
impl StateT for Follower {}
impl StateT for Candidate {}

#[derive(Debug)]
pub struct Server<S: StateT> {
    _state: std::marker::PhantomData<S>,
    name: usize,
    order_rx: Receiver<Order>,
    message_rx: Receiver<ServerMessage>,
    self_transmitter: Sender<ServerMessage>,
    pub neighbours: HashMap<usize, Sender<ServerMessage>>,
    info: ConsensusInfo,
    settings: ServerSettings,
}

mod candidate;
mod common;
mod follower;
mod initial;
mod leader;
