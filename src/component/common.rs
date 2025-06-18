use crate::component::{message::Message, StateTrait};

use super::{Component};

impl <T: StateTrait> Component <T> {
    pub fn neighbours_len(&self) -> usize {
        self.neighbours.len()
    }

    pub fn send_message(&self, message: Message, neighbour: usize)-> Result<(), crossbeam::channel::SendError<Message>> {
        self.neighbours[&neighbour].send(message)
    }

    pub fn open_message(&self) -> Message {
        self.rx.recv().unwrap()
    }

    pub fn get_name(&self) -> usize {
        self.name
    }

    pub fn yell(&self) {
    }
}
