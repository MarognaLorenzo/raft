use crate::component::ComponentState;

use super::{Component};

impl <T: ComponentState, S> Component <T, S> {
    pub fn neighbours_len(&self) -> usize {
        self.neighbours.len()
    }
}
