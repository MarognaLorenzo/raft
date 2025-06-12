use super::{Component, Candidate, Leader, Follower};
impl <MessageType>Component<Candidate, MessageType> {
   pub fn get_elected(self) -> Component<Leader, MessageType> {
        println!("My self {} got elected as Leader!", self.name);
        Component{
            _state: std::marker::PhantomData,
            name: self.name, 
            log: self.log,
            total_elements: self.total_elements,
            rx: self.rx,
            neighbours: self.neighbours,
        }
    }

    pub fn drop(self) -> Component<Follower, MessageType> {
        println!("Oh no your dropping me!");
        Component{
            _state: std::marker::PhantomData,
            name: self.name,
            log: self.log,
            total_elements: self.total_elements,
            rx: self.rx,
            neighbours: self.neighbours,
        }
    }
}
