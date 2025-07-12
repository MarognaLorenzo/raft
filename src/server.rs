use std::fmt::{Debug, Display};

// Import modules from the structures directory
pub mod structures {
    pub mod consensus_info;
    pub mod message;
    pub mod order;
    pub mod server_settings;
    pub mod components;
}

// Re-export commonly used types for convenience
pub use structures::consensus_info::ConsensusInfo;
pub use structures::message::ServerMessage;
pub use structures::order::Order;
pub use structures::server_settings::ServerSettings;
pub use structures::components::ServerComponents;

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
    info: ConsensusInfo,
    settings: ServerSettings,
     components: ServerComponents, 
}

mod candidate;
mod common;
mod follower;
mod initial;
mod leader;
