use crossbeam::channel::{select_biased, Receiver};

use crate::component::{message::ServerMessage, order::Order, ServerT, StateT};

use super::{Server};

impl <T: StateT> Server <T> {
    pub fn neighbours_len(&self) -> usize {
        self.neighbours.len()
    }

    pub fn send_message(&self, message: ServerMessage, neighbour: usize)-> Result<(), crossbeam::channel::SendError<ServerMessage>> {
        self.neighbours[&neighbour].send(message)
    }

    pub fn open_message(&self) -> ServerMessage {
        self.message_rx.recv().unwrap()
    }

    pub fn yell(&self) {
    }

    pub fn get_name(&self) -> usize {
        self.name
    }

    fn get_command_receiver(&self) -> &Receiver<Order> {
        &self.order_rx
    }

    fn get_network_receiver(&self) -> &Receiver<ServerMessage> {
        &self.message_rx
    }
}
impl<T> Server<T>
where 
    Server<T> : ServerT,
    T: StateT,
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
