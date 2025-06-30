use crate::component::{
    consensus_info::LogEntry, message::ServerMessage, order::Order, Candidate, ServerT,
};
use crossbeam::channel::{unbounded, Receiver};

use super::{Follower, Server};
impl Server<Follower> {
    pub fn to_candidate(self) -> Server<Candidate> {
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

    pub fn on_disconnect(mut self) -> (bool, Box<dyn ServerT>) {
        self.settings.activated = false;
        (false, Box::new(self))
    }
    pub fn on_connect(mut self) -> (bool, Box<dyn ServerT>) {
        self.settings.activated = true;
        (false, Box::new(self))
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

        log::debug!("{} is spawning a timer", self.name);
        self.update_timer(ServerMessage::TimerExpired, None);

        self.neighbours.values().for_each(|follower| follower.send(message.clone()).unwrap());
        Box::new(self.to_candidate())
    }
    fn on_vote_request(
        mut self,
        candidate_id: usize,
        candidate_term: usize,
        candidate_log_length: usize,
        candidate_log_term: usize,
    ) -> Box<dyn ServerT> {
        let _ = self.handle_vote_request(
            candidate_id,
            candidate_term,
            candidate_log_length,
            candidate_log_term,
        );
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
        self.handle_log_request(
            leader_id,
            leader_term,
            prefix_len,
            prefix_term,
            leader_commit,
            suffix,
        );
        Box::new(self)
    }

    fn on_send_info(mut self, msg: String) -> (bool, Box<dyn ServerT>) {
        if self.info.current_leader.is_none() {
            panic!("Follower {} has no leader", self.name);
        }
        let leader = self.info.current_leader.unwrap();
        let message = ServerMessage::SendInfo { msg: msg};
        self.send_message(message, leader).unwrap();
        return (false, Box::new(self));
    }


    fn on_log_response (mut self, _responser_id: usize, responder_term: usize, _ack: usize, _answer: bool) -> Box<dyn ServerT> {
        if responder_term > self.info.current_term {
            self.info.current_term = responder_term;
            self.info.voted_for = None;
            self.update_timer(ServerMessage::TimerExpired, None);
        }
        return Box::new(self);
    }

    fn on_vote_receive(mut self, _: usize, responder_term: usize, _: bool) -> Box<dyn ServerT> {
        if responder_term > self.info.current_term {
            self.info.current_term = responder_term;
            self.info.voted_for = None;
            self.update_timer(ServerMessage::TimerExpired, None);
        }
        Box::new(self)
    }
}

impl ServerT for Server<Follower> {
    fn handle_order(self: Box<Self>, order: Order) -> (bool, Box<dyn ServerT>) {
        match order {
            Order::SendInfo { info } => self.on_send_info(info),
            Order::Disconnect => self.on_disconnect(),
            Order::Reconnect => self.on_connect(),
            Order::Exit => (true, Box::new(*self)),
            Order::ConvertToFollower => (false, Box::new(*self)),
            Order::ConvertToCandidate => (false, Box::new((*self).to_candidate())),
            _ => (false, Box::new(*self)),
        }
    }

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
            ServerMessage::LogResponse {
                responder_id,
                responder_term,
                ack,
                answer
            } => self.on_log_response(responder_id, responder_term, ack, answer),
            ServerMessage::VoteResponse {
                responser_id,
                responder_term,
                response,
            } => self.on_vote_receive(responser_id, responder_term, response),
            _ => Box::new(*self),
        }
    }
}
