use crate::component::{order::Order, ServerT, };

use super::{Server, Candidate, Leader, Follower};
impl Server<Candidate> {
    
   pub fn candidate(&self){
        for neigh in self.neighbours.values() {
         neigh.send(super::message::ServerMessage::VoteRequest { 
                candidate_id: self.name, 
                candidate_term: self.term, 
                log_length: 0, 
                last_term: 0 
            }).unwrap();

        }
   }
   pub fn get_elected(self) -> Server<Leader> {
        println!("My self {} got elected as Leader!", self.name);
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

   pub fn to_follower(self) -> Server<Follower> {
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

    pub fn drop(self) -> Server<Follower> {
        println!("Oh no your dropping me!");
        Server{
            _state: std::marker::PhantomData,
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

impl ServerT for Server<Candidate>{
    fn handle_server_message(self: Box<Self>, message: super::message::ServerMessage) -> Box<dyn ServerT> {
        Box::new(*self)
    }

    fn handle_order(self: Box<Self>, order: Order) -> (bool, Box<dyn ServerT>) {
        match order {
        Order::SendInfo { info } => {
            println!("I am candidate {} and I received info {}", self.name, info);
            (false, Box::new(*self))
        }
        Order::Exit => (true, Box::new(*self)),
        Order::ConvertToFollower => (false, Box::new((*self).to_follower())),
        Order::ConvertToCandidate => (false, Box::new(*self)),
        }
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
