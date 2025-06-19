

use crossbeam::channel::{select_biased, Receiver, SendError};

use crate::component::{message::ServerMessage, order::Order, ServerT, StateT};

use super::{Server};

impl <T: StateT> Server <T> {
    pub fn neighbours_len(&self) -> usize {
        self.neighbours.len()
    }

    pub fn send_message(&self, message: ServerMessage, neighbour: usize)-> Result<(), SendError<ServerMessage>> {
        self.neighbours[&neighbour].send(message)
    }

    pub fn open_message(&self) -> ServerMessage {
        self.message_rx.recv().unwrap()
    }
    pub fn get_name(&self) -> usize {
        self.name
    }
}

impl<T> Server<T>
where 
    Server<T> : ServerT,
    T: StateT,
{
    pub fn activate(self) {
        let order_receiver = self.order_rx.clone();
        let message_receiver = self.message_rx.clone();

        let mut boxed: Box<dyn ServerT> = Box::new(self);
        loop {
            select_biased!(
                recv(message_receiver) -> mes => {
                    println!("ciao from me network {:?}!", boxed);
                    let next = boxed.handle_server_message(mes.unwrap());
                    boxed = next;
                }
                recv(order_receiver) -> mes => {
                    println!("ciao from me command {:?}!", boxed);
                    let (stop, next) = boxed.handle_order(mes.unwrap());
                    boxed = next;
                    if stop {
                        break;
                    }
                }
            )
        }
    }
}
