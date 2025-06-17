use crate::component::{message::Message, StateTrait};

use super::{Component};

impl <T: StateTrait> Component <T> {
    pub fn neighbours_len(&self) -> usize {
        self.neighbours.len()
    }

    pub async fn send_message(&self, message: Message, neighbour: usize) {
        self.neighbours[&neighbour].send(message).await.unwrap()
    }

    pub async fn open_message(&mut self) -> Message {
        self.rx.recv().await.unwrap()
    }

    pub fn get_name(&self) -> usize {
        self.name
    }

    pub fn yell(&self) {
    }
}
