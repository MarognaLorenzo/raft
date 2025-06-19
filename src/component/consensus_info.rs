use std::collections::HashMap;

use crossbeam::channel::Sender;

#[derive(Debug)]
pub struct ConsensusInfo {
    pub current_term: usize,
    pub voted_for: Option<usize>,
    pub log: Vec<LogEntry>,
    pub commit_length: usize,
    pub current_leader: usize,
    pub votes_received: Vec<usize>,
    pub sent_length: HashMap<usize, usize>,
    pub acked_length: HashMap<usize, usize>,
    pub leader_has_visited: bool,
    pub old_timer_tx: Option<Sender<()>>,
}

impl ConsensusInfo {
    pub fn new() -> ConsensusInfo {
        ConsensusInfo {
            current_term: 0,
            voted_for: None,
            log: vec![],
            commit_length: 0,
            current_leader: 0,
            votes_received: vec![],
            sent_length: HashMap::new(),
            acked_length: HashMap::new(),
            leader_has_visited: false,
            old_timer_tx: None,
        }
    }
}

#[derive(Debug)]
pub struct LogEntry {
    pub data: String,
    pub term: usize,
}
