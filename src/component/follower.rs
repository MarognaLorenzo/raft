use crossbeam::channel::Receiver;

use crate::component::{message::ServerMessage, order::Order, ServerT};

use super::{Server, Follower};
impl Server<Follower> {
    pub fn lament (self) {
        println!("Oh no I am broken forever");
    }
}

impl ServerT for Server<Follower>{
    fn handle_order(self: Box<Self>, order: Order) -> (bool, Box<dyn ServerT>) {
        (true, Box::new(*self)) 
    }
    fn handle_server_message(self: Box<Self>, message: super::message::ServerMessage) -> Box<dyn ServerT> {
        Box::new(*self)
    }
}
