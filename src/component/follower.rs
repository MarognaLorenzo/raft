use crossbeam::channel::Receiver;

use crate::component::{message::ServerMessage, order::Order, Candidate, ServerT};

use super::{Server, Follower};
impl Server<Follower> {
    pub fn lament (self) {
        println!("Oh no I am broken forever");
    }

    pub fn to_candidate(self) -> Server<Candidate>{
        Server{
            _state: std::marker::PhantomData,
            name: self.name, 
            log: self.log,
            total_elements: self.total_elements,
            message_rx: self.message_rx,
            order_rx: self.order_rx,
            neighbours: self.neighbours,
            term: self.term,
            voted_for: self.voted_for,
        }
    }
}

impl ServerT for Server<Follower>{
    fn handle_order(self: Box<Self>, order: Order) -> (bool, Box<dyn ServerT>) {
        match order {
        Order::SendInfo { info } => {
            println!("I am candidate {} and I received info {}", self.name, info);
            (false, Box::new(*self))
        }
        Order::Exit => (true, Box::new(*self)),
        Order::ConvertToFollower => (false, Box::new(*self)),
        Order::ConvertToCandidate => (false, Box::new((*self).to_candidate())),
        }
    }
    fn handle_server_message(self: Box<Self>, message: super::message::ServerMessage) -> Box<dyn ServerT> {
        Box::new(*self)
    }
}
