use crossbeam::channel::{select_biased, Receiver};

use crate::component::{message::ComponentMessage, order::Order, ComponentTrait, StateTrait};

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

    pub fn yell(&self) {
    }

    pub fn get_name(&self) -> usize {
        self.name
    }

    fn get_command_receiver(&self) -> &Receiver<Order> {
        &self.command_rx
    }

    fn get_network_receiver(&self) -> &Receiver<ComponentMessage> {
        &self.network_rx
    }
}
impl<T> Component<T>
where 
    Component<T> : ComponentTrait,
    T: StateTrait,
{
    pub fn activate(&self) {
        loop {
            select_biased!(
                recv(self.get_command_receiver()) -> mes => {
                    println!("ciao from me command {}!", self.get_name());
                    if self.handle_order(mes.unwrap()) {
                        break;
                    }
                }
                recv(self.get_network_receiver()) -> mes => {
                    println!("ciao from me network {}!", self.get_name());
                    self.handle_component_message(mes.unwrap());
                }
            )
        }
    }
}
