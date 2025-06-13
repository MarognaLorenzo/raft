use super::{Leader, Component, Candidate};
impl Component<Leader> {
    pub fn turn_on(self) -> Component<Candidate> {
        println!("Swithching on!");
        Component{
            _state : std::marker::PhantomData,
            name: self.name,
            log: self.log,
            total_elements: self.total_elements,
            rx: self.rx,
            neighbours: self.neighbours,
            term: self.term,
            voted_for: self.voted_for,
        }
    }
}


