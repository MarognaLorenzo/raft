use super::{Leader, Component, Candidate};
impl <MessageType>Component<Leader, MessageType> {
    pub fn turn_on(self) -> Component<Candidate, MessageType> {
        println!("Swithching on!");
        Component{
            _state : std::marker::PhantomData,
            name: self.name,
            log: self.log,
            total_elements: self.total_elements,
            rx: self.rx,
            neighbours: self.neighbours,
        }
    }
}


