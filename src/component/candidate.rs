use std::collections::HashMap;
use std::path::Component;
use std::thread;
use std::time::Duration;

use crossbeam::channel::unbounded;
use crossbeam::channel::{self, Receiver, Sender};
use crossbeam::select;

use crate::component::consensus_info::LogEntry;
use crate::component::StateT;
use crate::component::{common, message::ServerMessage, order::Order, ServerT};

use super::{Candidate, Follower, Leader, Server};
impl Server<Candidate> {
    pub fn candidate(&self) {
        for neigh in self.neighbours.values() {
            neigh
                .send(super::message::ServerMessage::VoteRequest {
                    candidate_id: self.name,
                    candidate_term: self.info.current_term,
                    log_length: 0,
                    last_term: 0,
                })
                .unwrap();
        }
    }

    pub fn on_disconnect(mut self) -> (bool, Box<dyn ServerT>) {
        self.settings.activated = false;
        (false, Box::new(self))
    }
    pub fn on_connect(mut self) -> (bool, Box<dyn ServerT>) {
        self.settings.activated = true;
        (false, Box::new(self))
    }

    pub fn to_leader(self) -> Server<Leader> {
        Server {
            _state: std::marker::PhantomData,
            name: self.name,
            total_elements: self.total_elements,
            message_rx: self.message_rx,
            order_rx: self.order_rx,
            self_transmitter: self.self_transmitter,
            neighbours: self.neighbours,
            info: self.info,
            settings: self.settings,
        }
    }

    pub fn to_follower(self) -> Server<Follower> {
        Server {
            _state: std::marker::PhantomData,
            name: self.name,
            total_elements: self.total_elements,
            message_rx: self.message_rx,
            order_rx: self.order_rx,
            self_transmitter: self.self_transmitter,
            neighbours: self.neighbours,
            info: self.info,
            settings: self.settings,
        }
    }

    fn on_heartbeat_received(mut self, leader_id: usize, current_term: usize) -> Box<dyn ServerT> {
        self.update_timer(ServerMessage::TimerExpired, None);
        Box::new(self)
    }

    fn on_log_request_received(
        mut self,
        leader_id: usize,
        leader_term: usize,
        prefix_len: usize,
        prefix_term: usize,
        leader_commit: usize,
        suffix: Vec<LogEntry>,
    ) -> Box<dyn ServerT> {
        let change_to_follower = self.handle_log_request(
            leader_id,
            leader_term,
            prefix_len,
            prefix_term,
            leader_commit,
            suffix,
        );
        if change_to_follower {
            Box::new(self.to_follower())
        } else {
            Box::new(self)
        }
    }

    fn on_timer_expired(mut self) -> Box<dyn ServerT> {
        self.info.current_term += 1;
        //convert to candidate
        self.info.voted_for = Some(self.name);
        self.info.votes_received = vec![self.name];
        let last_term = match self.info.log.last() {
            Some(entry) => entry.term,
            None => 0,
        };

        let message = ServerMessage::VoteRequest {
            candidate_id: self.name,
            candidate_term: self.info.current_term,
            log_length: self.info.log.len(),
            last_term: last_term,
        };

        println!("{} is spawning a timer", self.name);
        self.update_timer(ServerMessage::TimerExpired, None);
        self.neighbours.values().for_each(|follower| follower.send(message.clone()).unwrap());
        Box::new(self)
    }

    fn on_vote_request(
        mut self,
        candidate_id: usize,
        candidate_term: usize,
        candidate_log_length: usize,
        candidate_log_term: usize,
    ) -> Box<dyn ServerT> {
        let change_to_follower = self.handle_vote_request(
            candidate_id,
            candidate_term,
            candidate_log_length,
            candidate_log_term,
        );
        if change_to_follower {
            Box::new(self.to_follower())
        } else {
            Box::new(self)
        }
    }

    fn on_send_info(self, msg: String) -> (bool, Box<dyn ServerT>) {
        let sender = self.get_self_sender().clone();
        thread::spawn(move ||{
            thread::sleep(Duration::from_secs(1));
            sender.send(ServerMessage::SendInfo { msg: msg }).unwrap();
        });
        (false, Box::new(self))
    }

    fn on_log_response (mut self, _responser_id: usize, responder_term: usize, _ack: usize, _answer: bool) -> Box<dyn ServerT> {
        if responder_term > self.info.current_term {
            self.info.current_term = responder_term;
            self.info.voted_for = None;
            self.update_timer(ServerMessage::TimerExpired, None);
        }
        return Box::new(self.to_follower());
    }

    fn on_vote_receive(
        mut self,
        responser_id: usize,
        responder_term: usize,
        response: bool,
    ) -> Box<dyn ServerT> {
        if responder_term == self.info.current_term && response {
            self.info.votes_received.push(responser_id);
            let quorum = (self.total_elements + 1).div_ceil(2) as usize;
            if self.info.votes_received.len() >= quorum {
                self.info.current_leader = Some(self.name);
                println!(
                    "\n {} got elected to leader {:?}",
                    self.name, self.info.votes_received
                );
                println!("{} about to be elected leader: {:?}", self.name, self.settings );
                let neighs:Vec<_> = self.neighbours.keys().copied().collect();
                for follower in neighs{
                    self.info.sent_length.insert(follower, self.info.log.len());
                    self.info.acked_length.insert(follower, 0);
                    self.replicate_log(follower);
                    println!("{} Sent log to {}", self.name, follower);
                }
                self.self_transmitter.send(ServerMessage::SendHeartBeat).unwrap();
                Box::new(self.to_leader())
            } else {
                // Not yet elected
                Box::new(self)
            }
        } else if responder_term > self.info.current_term {
            self.info.current_term = responder_term;
            self.info.voted_for = None;
            self.update_timer(ServerMessage::TimerExpired, None);
            Box::new(self.to_follower())
        } else {
            // Ignore the message
            Box::new(self)
        }
    }
}

impl ServerT for Server<Candidate> {
    fn handle_server_message(
        mut self: Box<Self>,
        message: super::message::ServerMessage,
    ) -> Box<dyn ServerT> {
        match message {
            ServerMessage::LogRequest {
                leader_id,
                current_term,
                prefix_len,
                prefix_term,
                commit_length,
                suffix,
            } => self.on_log_request_received(
                leader_id,
                current_term,
                prefix_len,
                prefix_term,
                commit_length,
                suffix,
            ),
            ServerMessage::HeartBeatSent {
                leader_id,
                current_term,
            } => self.on_heartbeat_received(leader_id, current_term),
            ServerMessage::TimerExpired => self.on_timer_expired(),
            ServerMessage::VoteRequest {
                candidate_id,
                candidate_term,
                log_length,
                last_term,
            } => self.on_vote_request(candidate_id, candidate_term, log_length, last_term),
            ServerMessage::VoteResponse {
                responser_id,
                responder_term,
                response,
            } => self.on_vote_receive(responser_id, responder_term, response),
            ServerMessage::LogResponse {
                responder_id,
                responder_term,
                ack,
                answer
            } => self.on_log_response(responder_id, responder_term, ack, answer),
            ServerMessage::SendInfo { msg } => self.on_send_info(msg).1,
            _ => Box::new(*self),
        }
    }

    fn handle_order(self: Box<Self>, order: Order) -> (bool, Box<dyn ServerT>) {
        match order {
            Order::Disconnect => self.on_disconnect(),
            Order::Reconnect => self.on_connect(),
            Order::SendInfo { info } => self.on_send_info(info),
            Order::Exit => (true, Box::new(*self)),
            Order::ConvertToFollower => (false, Box::new((*self).to_follower())),
            Order::ConvertToCandidate => (false, Box::new(*self)),
            _ => (false, Box::new(*self)),
        }
    }
}
