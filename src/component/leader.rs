use crate::component::{order::Order, ServerT};

use super::{Leader, Server, Candidate};

impl Server<Leader> {
    pub fn turn_on(self) -> Server<Candidate> {
        println!("Swithching on!");
        Server{
            _state : std::marker::PhantomData,
            name: self.name,
            log: self.log,
            total_elements: self.total_elements,
            order_rx: self.order_rx,
            message_rx: self.message_rx,
            neighbours: self.neighbours,
            term: self.term,
            voted_for: self.voted_for,
        }
    }
}


impl ServerT for Server<Leader>{
    fn handle_order(self: Box<Self>, order: Order) -> (bool, Box<dyn ServerT>) {
        (true, Box::new(*self)) 
    }
    fn handle_server_message(self: Box<Self>, message: super::message::ServerMessage) -> Box<dyn ServerT> {
        Box::new(*self)
    }
    /* fn handle_order(&self, order: super::order::Order) -> bool {
        match order {
            Order::SendInfo { info } => {
                println!("Received: {}", info);
                true
            }
        }
    } */
}
