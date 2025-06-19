use std::thread;
use std::time::Duration;

use crossbeam::channel::{self, Receiver, Sender};
use crossbeam::channel::unbounded;
use crossbeam::select;

use crate::component::{message::ServerMessage, order::Order, ServerT };

use super::{Server, Candidate, Leader, Follower};
impl Server<Candidate> {
    
   pub fn candidate(&self){
        for neigh in self.neighbours.values() {
         neigh.send(super::message::ServerMessage::VoteRequest { 
                candidate_id: self.name, 
                candidate_term: self.info.current_term, 
                log_length: 0, 
                last_term: 0 
            }).unwrap();

        }
   }
   pub fn get_elected(self) -> Server<Leader> {
        println!("My self {} got elected as Leader!", self.name);
        Server{
            _state: std::marker::PhantomData,
            name: self.name, 
            total_elements: self.total_elements,
            message_rx: self.message_rx,
            order_rx: self.order_rx,
            neighbours: self.neighbours,
            info: self.info,
        }
    }

   pub fn to_follower(self) -> Server<Follower> {
        Server{
            _state: std::marker::PhantomData,
            name: self.name, 
            total_elements: self.total_elements,
            message_rx: self.message_rx,
            order_rx: self.order_rx,
            neighbours: self.neighbours,
            info: self.info
        }
   }

   fn spawn_timer(stop_recv: Receiver<()>, expiration_tx: Sender<ServerMessage>){
        thread::spawn(move ||{
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
   }
   fn on_heartbeat_received(&mut self, leader_id: usize, current_term: usize){
        if let Some(old_timer) = self.info.old_timer_tx.take() {
            old_timer.send(()).unwrap();
        }
       let (stop_timer_tx, stop_timer_rx) = unbounded::<()>();
       self.info.old_timer_tx = Some(stop_timer_tx);

       Self::spawn_timer(stop_timer_rx, self.neighbours[&self.name].clone());
   }
}

impl ServerT for Server<Candidate>{
    fn handle_server_message(mut self: Box<Self>, message: super::message::ServerMessage) -> Box<dyn ServerT> {
        match message {
            ServerMessage::HeartBeatSent { leader_id, current_term } => self.on_heartbeat_received(leader_id, current_term),
            _ => {},
        };
        Box::new(*self)
    }

    fn handle_order(self: Box<Self>, order: Order) -> (bool, Box<dyn ServerT>) {
        match order {
        Order::SendInfo { info } => {
            println!("I am candidate {} and I received info {}", self.name, info);
            (false, Box::new(*self))
        }
        Order::Exit => (true, Box::new(*self)),
        Order::ConvertToFollower => (false, Box::new((*self).to_follower())),
        Order::ConvertToCandidate => (false, Box::new(*self)),
        }
    }
    /* fn handle_order(&self, order: super::order::Order) -> bool {
        match order {
            Order::SendInfo { info } => {
                println!("Received: {}", info);
                true
            }
        }
    } */
}
