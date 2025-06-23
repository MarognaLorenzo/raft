use crate::component::{message::ServerMessage, order::Order, Follower, ServerT};

use super::{Leader, Server};

impl Server<Leader> {
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
        if self.settings.activated {
            self.update_timer(ServerMessage::SendHeartBeat, Some(2));
            self.neighbours.keys().for_each(|&follower| {
                self.replicate_log(follower);
            });
        }
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
}

impl ServerT for Server<Leader> {
    fn handle_order(self: Box<Self>, order: Order) -> (bool, Box<dyn ServerT>) {
        match order {
            Order::Disconnect => self.on_disconnect(),
            Order::Reconnect => self.on_connect(),
            _ => (false, Box::new(*self)),
            

        }
    }
    fn handle_server_message(self: Box<Self>, message: ServerMessage) -> Box<dyn ServerT> {
        match message {
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
