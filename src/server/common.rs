use log::{info, log};
use rand::{random, rng, Rng};
use std::{cmp::min, fmt, thread, time::Duration};
use rand_distr::{Distribution, Normal, NormalError};
use crossbeam::{
    channel::{select_biased, unbounded, SendError, SendTimeoutError, Sender},
    select,
};

use crate::server::{
    structures::{consensus_info::LogEntry, message::ServerMessage}, ServerT, StateT,
};

use super::Server;

impl<T: StateT> Server<T> {
    pub fn neighbours_len(&self) -> usize {
        self.components.neighbours.len()
    }

    pub fn send_message(
        &self,
        message: ServerMessage,
        neighbour: usize,
    ) -> Result<(), ServerMessage> {
        let mut rng = rng();
        let distribution = rand_distr::Exp::new(0.5).unwrap();
        let delay = distribution.sample(&mut rng);
        thread::sleep(Duration::from_millis(delay as u64));
        if self.components.neighbours[&neighbour].send(message.clone()).is_err() {
            return Err(message);
        } else {
            return Ok(());
        }

    }

    pub fn open_message(&self) -> ServerMessage {
        self.components.message_rx.recv().unwrap()
    }
    pub fn get_name(&self) -> usize {
        self.name
    }
    pub fn get_self_sender(&self) -> &Sender<ServerMessage> {
        &self.components.self_transmitter
    }

    pub fn update_timer(&mut self, message: ServerMessage, time: Option<usize>) {
        // cancel election timer
        if let Some(old_timer) = self.info.old_timer_tx.take() {
            old_timer.send(()).unwrap_or_default();
        }

        let (stop_send, stop_recv) = unbounded();
        self.info.old_timer_tx = Some(stop_send);
        let sender = self.get_self_sender().clone();
        let waiting_time = time.unwrap_or(match message {
            ServerMessage::TimerExpired => self.settings.election_timeout,
            ServerMessage::SendHeartBeat => self.settings.heartbeat_timeout,
            _ => 10,
        });
        let _ = thread::Builder::new()
            .name("Timer".to_string())
            .spawn(move || {
                select! {
                    recv(stop_recv) -> _ => {
                        // timer cancelled
                        return;
                    }
                    default(Duration::from_secs(time.unwrap_or(waiting_time) as u64)) => {
                        //timeout elapsed
                        sender.send(message).unwrap();
                    }
                }
            });
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
        let log_ok: bool = (candidate_log_term > last_term)
            || (candidate_log_term == last_term && candidate_log_length >= self.info.log.len());
        let voted_ok: bool = self
            .info
            .voted_for
            .is_none_or(|voted| voted == candidate_id);
        let answer: bool = candidate_term == self.info.current_term && log_ok && voted_ok;
        if answer {
            self.info.voted_for = Some(candidate_id);
        }
        let accepted_request = ServerMessage::VoteResponse {
            responser_id: self.name,
            responder_term: self.info.current_term,
            response: answer,
        };
        if !answer {
            log::debug!(
                "{} answered no to {}. (term {}) (logOk {}) (voted_ok {}) - voted => {:?}",
                self.name,
                candidate_id,
                self.info.current_term == candidate_term,
                log_ok,
                voted_ok,
                self.info.voted_for
            );
        } else {
            log::debug!("{} answered yes to {}", self.name, candidate_id);
        }
        self.send_message(accepted_request, candidate_id).unwrap();

        //return true if the server needs to turn into follower mode
        return received_newer_term;
    }

    pub fn handle_list_log(&self ){
        let log : Vec<_> = self.info.log.iter().map(|entry| entry.data.clone()).collect();
        let log = log.join("\n");
        info!("Log of {}: \n  {}", self.name, log);
    }

    pub fn handle_log_request(
        &mut self,
        leader_id: usize,
        leader_term: usize,
        prefix_len: usize,
        prefix_term: usize,
        leader_commit: usize,
        suffix: Vec<LogEntry>,
    ) -> bool {
        // Change to return true if need to change to follower ->
        // let mut
        log::debug!(
            "{} received a log request from term {} and is in term {}",
            self.name, leader_term, self.info.current_term
        );
        if leader_term > self.info.current_term {
            self.info.current_term = leader_term;
            self.info.voted_for = None;
        }

        // return true if I want to go into follower state:
        let equal_term: bool = leader_term == self.info.current_term;
        if equal_term {
            self.info.current_leader = Some(leader_id);
            self.update_timer(ServerMessage::TimerExpired, None);
        }

        let log_ok: bool = prefix_len == 0
            || self
                .info
                .log
                .get(prefix_len - 1)
                .is_some_and(|entry| entry.term == prefix_term);
        let answer: bool = self.info.current_term == leader_term && log_ok;
        let message = if answer {
            let ack = prefix_len + suffix.len();
            self.append_entries(prefix_len, leader_commit, suffix);
            ServerMessage::LogResponse {
                responder_id: self.name,
                responder_term: self.info.current_term,
                ack: ack,
                answer: true,
            }
            //do stuff
        } else {
            ServerMessage::LogResponse {
                responder_id: self.name,
                responder_term: self.info.current_term,
                ack: 0,
                answer: false,
            }
        };

        self.send_message(message, leader_id).unwrap();
        return equal_term;
    }

    fn append_entries(&mut self, prefix_len: usize, leader_commit: usize, suffix: Vec<LogEntry>) {
        if !suffix.is_empty() && self.info.log.len() > prefix_len {
            let index = min(self.info.log.len(), prefix_len + suffix.len())-1;
            if self.info.log[index].term != suffix[index - prefix_len].term {
                self.info.log.truncate(prefix_len);
            }
        }
        if prefix_len + suffix.len() > self.info.log.len() {
            let skip = self.info.log.len().saturating_sub(prefix_len);
            self.info.log.extend(suffix.iter().skip(skip).cloned());
        }
        if leader_commit > self.info.commit_length {
            self.info.commit_length = leader_commit;
            // send all info from commitLength to leader_commit to the application.
        }
    }

    pub fn replicate_log(&self, follower_id: usize) {
        let prefix_len = self.info.sent_length[&follower_id];
        let suffix: Vec<LogEntry> = self.info.log[prefix_len..].to_vec();
        let prefix_term = match self.info.log.last() {
            Some(entry) => entry.term,
            None => 0,
        };
        let message = ServerMessage::LogRequest {
            leader_id: self.name,
            current_term: self.info.current_term,
            prefix_len: prefix_len,
            prefix_term: prefix_term,
            commit_length: self.info.commit_length,
            suffix: suffix,
        };
        self.send_message(message, follower_id).unwrap();
    }
}

impl<T> Server<T>
where
    Server<T>: ServerT,
    T: StateT,
{
    pub fn activate(self) {
        let order_receiver = self.components.order_rx.clone();
        let message_receiver = self.components.message_rx.clone();

        let micros = rand::rng().random_range(50..400);
        thread::sleep(Duration::from_micros(micros));

        let mut boxed: Box<dyn ServerT> = Box::new(self);
        loop {
            select_biased!(
                recv(message_receiver) -> mes => {
                    let message = mes.unwrap();
                    log::debug!("network - {} received {:?}", boxed, message);
                    let next = boxed.handle_server_message(message);
                    boxed = next;
                }
                recv(order_receiver) -> mes => {
                    let message = mes.unwrap();
                    log::debug!("command - {} received {:?}", boxed, message);
                    let (stop, next) = boxed.handle_order(message);
                    boxed = next;
                    if stop {
                        break;
                    }
                }
            )
        }
    }


}

impl<T: StateT> fmt::Display for Server<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write to the formatter using `write!` macro
        write!(f, "({}, {:?})", self.name, self._state)
    }
}
