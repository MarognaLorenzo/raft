use crossbeam::channel::select_biased;

use crate::component::{message::ComponentMessage, StateTrait};

use super::{Component};

impl <T: StateTrait> Component <T> {
    pub fn neighbours_len(&self) -> usize {
        self.neighbours.len()
    }

    pub fn send_message(&self, message: ComponentMessage, neighbour: usize)-> Result<(), crossbeam::channel::SendError<ComponentMessage>> {
        self.neighbours[&neighbour].send(message)
    }

    pub fn open_message(&self) -> ComponentMessage {
        self.network_rx.recv().unwrap()
    }

    pub fn get_name(&self) -> usize {
        self.name
    }

    pub fn activate(&self) {
        loop {
            select_biased!(
                recv(self.command_rx) -> mes => {print!("ciao from me command {}!", self.get_name())}
                recv(self.network_rx) -> mes => {print!("ciao from me network {}!", self.get_name()); break;}
            )
        }
    }

    pub fn yell(&self) {
    }
}
