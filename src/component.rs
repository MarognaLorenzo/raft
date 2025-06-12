use std::sync::mpsc::{Receiver, Sender};

pub struct Initial;
pub struct Leader;
pub struct Candidate;
pub struct Follower;

pub trait ComponentState{}

impl ComponentState for Initial {}
impl ComponentState for Leader {}
impl ComponentState for Follower {}
impl ComponentState for Candidate {}

pub struct Component<S: ComponentState, MessageType> {
    _state: std::marker::PhantomData<S>,
    log: Vec<i32>,
    name: i32,
    total_elements: i32,
    rx: Receiver<MessageType>,
    neighbours: Vec<Sender<MessageType>>
}

mod initial;
mod candidate;
mod leader;
mod follower;
mod common;

