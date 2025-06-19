use crate::component::{order::Order, ComponentTrait};

use super::{Component, Candidate, Leader, Follower};
impl Component<Candidate> {
    
   pub fn candidate(&self){
        for neigh in self.neighbours.values() {
         neigh.send(super::message::ComponentMessage::VoteRequest { 
                candidate_id: self.name, 
                candidate_term: self.term, 
                log_length: 0, 
                last_term: 0 
            }).unwrap();

        }
   }
   pub fn get_elected(self) -> Component<Leader> {
        println!("My self {} got elected as Leader!", self.name);
        Component{
            _state: std::marker::PhantomData,
            name: self.name, 
            log: self.log,
            total_elements: self.total_elements,
            network_rx: self.network_rx,
            command_rx: self.command_rx,
            neighbours: self.neighbours,
            term: self.term,
            voted_for: self.voted_for,
        }
    }

    pub fn drop(self) -> Component<Follower> {
        println!("Oh no your dropping me!");
        Component{
            _state: std::marker::PhantomData,
            name: self.name,
            log: self.log,
            total_elements: self.total_elements,
            command_rx: self.command_rx,
            network_rx: self.network_rx,
            neighbours: self.neighbours,
            term: self.term,
            voted_for: self.voted_for,
        }
    }
}

impl ComponentTrait for Component<Candidate>{
    fn handle_order(&self, order: super::order::Order) -> bool {
        match order {
            Order::SendInfo { info } => {
                println!("Received: {}", info);
                true
            }
        }
    }
}
