use std::{collections::HashMap, };

pub mod message;
use message::Message;
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
    rx: Receiver<Message>,
    pub neighbours: HashMap<usize, Sender<Message>>,
    term: usize,
    voted_for: Option<usize>,
}


mod initial;
mod candidate;
mod leader;
mod follower;
mod common;

