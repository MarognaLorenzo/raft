use crate::component::ComponentState;

use super::{Component};

impl <T: ComponentState, S> Component <T, S> {
    pub fn neighbours_len(&self) -> usize {
        self.neighbours.len()
    }

    pub fn send_message(&self, message: S, neighbour: usize) {
        self.neighbours[&neighbour].send(message).unwrap()
    }

    pub fn open_message(&self) -> S {
        self.rx.recv().unwrap()
    }
}
