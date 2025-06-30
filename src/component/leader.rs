use crate::component::{consensus_info::LogEntry, message::ServerMessage, order::Order, Follower, ServerT};

use super::{Leader, Server};

impl Server<Leader> {
    pub fn to_follower(self) -> Server<Follower> {
        Server {
            _state: std::marker::PhantomData,
            name: self.name,
            message_rx: self.message_rx,
            order_rx: self.order_rx,
            self_transmitter: self.self_transmitter,
            neighbours: self.neighbours,
            info: self.info,
            settings: self.settings,
        }
    }
    
    fn broadcast_replicate_log(&mut self) {
        if self.settings.activated {
            self.update_timer(ServerMessage::SendHeartBeat, Some(2));
            self.neighbours.keys().for_each(|&follower| {
                self.replicate_log(follower);
            });
        }
    }

    fn on_list_log(self) -> (bool, Box<dyn ServerT>) {
        self.handle_list_log();
        return (false, Box::new(self));
    }

    fn on_send_info(mut self, msg:String) -> (bool, Box<dyn ServerT>) {
        let entry = LogEntry { data: msg, term: self.info.current_term };
        self.info.log.push(entry);
        self.info.acked_length.insert(self.name, self.info.log.len());
        self.broadcast_replicate_log();
        return (false, Box::new(self))
    }

    fn on_vote_receive(mut self, _: usize, responder_term: usize, _: bool) -> Box<dyn ServerT> {
        if responder_term > self.info.current_term {
            self.info.current_term = responder_term;
            self.info.voted_for = None;
            self.update_timer(ServerMessage::TimerExpired, None);
            Box::new(self.to_follower())
        } else {
            Box::new(self)
        }
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

    fn on_send_heartbeat(mut self) -> Box<dyn ServerT> {
        self.broadcast_replicate_log();
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

    pub fn on_log_response(
        mut self,
        responder_id: usize,
        responder_term: usize,
        ack: usize,
        answer: bool
    ) -> Box<dyn ServerT> {
        if self.info.current_term == responder_term {
            if answer && ack >= self.info.acked_length[&responder_id] {
                self.info.sent_length.insert(responder_id, ack);
                self.info.acked_length.insert(responder_id, ack);
                self.commit_log_entries();
            } else if self.info.sent_length[&responder_id] > 0 {
                self.info.sent_length.entry(responder_id).and_modify(|v| *v-=1 );
                self.replicate_log(responder_id);
            }
        } else if responder_term > self.info.current_term {
            self.info.current_term = responder_term;
            self.info.voted_for = None;
            self.update_timer(ServerMessage::TimerExpired, None);
            return Box::new(self.to_follower());
        }
        return Box::new(self);
    }


    fn acks(&self, len: usize) -> usize {
        self.info.acked_length
            .values()
            .filter(|&acked_len| *acked_len >= len)
            .count()
    }

    pub fn commit_log_entries(&mut self) {
        let min_acks = (self.settings.total_elements + 1).div_ceil(2) as usize;
        let max_ready = (1..self.info.log.len())
            .rev()
            .find(|len| self.acks(*len) >= min_acks);

        if let Some(ready) = max_ready {
            if ready > self.info.commit_length && self.info.log[ready-1].term == self.info.current_term {
                for log in self.info.log
                    .iter()
                    .take(ready - 1)
                    .skip(self.info.commit_length)
                {
                    log::info!("{} delivers message to application: {}", self.name, log.data)
                };
                self.info.commit_length = ready;
            }
        }
    }

}

impl ServerT for Server<Leader> {
    fn handle_order(self: Box<Self>, order: Order) -> (bool, Box<dyn ServerT>) {
        match order {
            Order::Disconnect => self.on_disconnect(),
            Order::ListLog => self.on_list_log(),
            Order::Reconnect => self.on_connect(),
            Order::SendInfo { info } => self.on_send_info(info),
            _ => (false, Box::new(*self)),
        }
    }
    fn handle_server_message(self: Box<Self>, message: ServerMessage) -> Box<dyn ServerT> {
        match message {
            ServerMessage::LogResponse {
                responder_id,
                responder_term,
                ack,
                answer
            } => self.on_log_response(responder_id, responder_term, ack, answer),
            ServerMessage:: SendInfo { msg } => self.on_send_info(msg).1,
            ServerMessage::VoteResponse {
                responser_id,
                responder_term,
                response,
            } => self.on_vote_receive(responser_id, responder_term, response),
            ServerMessage::VoteRequest {
                candidate_id,
                candidate_term,
                log_length,
                last_term,
            } => self.on_vote_request(candidate_id, candidate_term, log_length, last_term),
            ServerMessage::SendHeartBeat => self.on_send_heartbeat(),
            _ => Box::new(*self),
        }
    }
}
