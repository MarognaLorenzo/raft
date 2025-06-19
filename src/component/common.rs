use std::{collections::HashMap, thread, time::Duration};

use crossbeam::{
    channel::{select_biased, unbounded, Receiver, SendError, Sender},
    select,
};

use crate::component::{message::ServerMessage, order::Order, ServerT, StateT};

use super::Server;

impl<T: StateT> Server<T> {
    pub fn neighbours_len(&self) -> usize {
        self.neighbours.len()
    }

    pub fn send_message(
        &self,
        message: ServerMessage,
        neighbour: usize,
    ) -> Result<(), SendError<ServerMessage>> {
        self.neighbours[&neighbour].send(message)
    }

    pub fn open_message(&self) -> ServerMessage {
        self.message_rx.recv().unwrap()
    }
    pub fn get_name(&self) -> usize {
        self.name
    }
    pub fn get_self_sender(&self) -> &Sender<ServerMessage> {
        &self.neighbours[&self.name]
    }
    pub fn broadcast<F>(&mut self, func: F)
    where
        F: Fn((&usize, &Sender<ServerMessage>)),
    {
        self.neighbours
            .iter()
            .filter(|(&k, _)| k != self.name)
            .for_each(func);
    }
    pub fn spawn_timer(expiration_tx: Sender<ServerMessage>) -> Sender<()> {
        let (stop_send, stop_recv) = unbounded();
        thread::spawn(move || {
            select! {
                recv(stop_recv) -> _ => {
                    // timer cancelled
                    return;
                }
                default(Duration::from_secs(3)) => {
                    //timeout elapsed
                    expiration_tx.send(
                        ServerMessage::TimerExpired
                    ).unwrap();
                }
            }
        });
        return stop_send;
    }

    pub fn handle_vote_request(
        &mut self,
        candidate_id: usize,
        candidate_term: usize,
        candidate_log_length: usize,
        candidate_log_term: usize,
    ) -> bool {
        let received_newer_term: bool = candidate_term > self.info.current_term;
        if received_newer_term {
            self.info.current_term = candidate_term;
            self.info.voted_for = None;
        }
        let last_term = match self.info.log.last() {
            Some(entry) => entry.term,
            None => 0,
        };
        let logOk: bool = (candidate_log_term > last_term)
            || (candidate_log_term == last_term && candidate_log_length > self.info.log.len());
        let voted_ok: bool = self
            .info
            .voted_for
            .is_none_or(|voted| voted == candidate_id);
        let answer: bool = candidate_term == self.info.current_term && logOk && voted_ok;
        let accepted_request = ServerMessage::VoteResponse {
            responser_id: self.name,
            responder_term: self.info.current_term,
            response: answer,
        };
        self.send_message(accepted_request, candidate_id).unwrap();

        //return true if the server needs to turn into follower mode
        return received_newer_term;
    }
}

impl<T> Server<T>
where
    Server<T>: ServerT,
    T: StateT,
{
    pub fn activate(mut self) {
        let order_receiver = self.order_rx.clone();
        let message_receiver = self.message_rx.clone();

        let timer_sender = Self::spawn_timer(self.get_self_sender().clone());
        self.info.old_timer_tx = Some(timer_sender);

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
