use std::{collections::HashMap, };

pub mod order;
use order::Order;
pub mod message;
use message::ComponentMessage;
use crossbeam::channel::*;

pub struct Initial;
pub struct Leader;
pub struct Candidate{
    voting_received: usize,
}
pub struct Follower{
    leader: usize,
}

pub trait StateTrait{}

impl StateTrait for Initial {}
impl StateTrait for Leader {}
impl StateTrait for Follower {}
impl StateTrait for Candidate {}

pub struct Component<S: StateTrait> {
    _state: std::marker::PhantomData<S>,
    log: Vec<i32>,
    name: usize,
    total_elements: usize,
    command_rx: Receiver<Order>,
    network_rx: Receiver<ComponentMessage>,
    pub neighbours: HashMap<usize, Sender<ComponentMessage>>,
    term: usize,
    voted_for: Option<usize>,
}


mod initial;
mod candidate;
mod leader;
mod follower;
mod common;

