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


