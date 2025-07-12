use crate::server::structures::consensus_info::LogEntry;

#[derive(Debug, Clone)]
pub enum ServerMessage {
    Ping {
        from: usize,
        to: usize,
    },
    
    SendInfo {
        msg: String,
    },

    ForwardInfo,

    VoteRequest {
        candidate_id: usize,
        candidate_term: usize,
        log_length: usize,
        last_term: usize,
    },

    VoteResponse {
        responser_id: usize,
        responder_term: usize,
        response: bool,
    },

    HeartBeatSent {
        leader_id: usize,
        current_term: usize,
    },

    SendHeartBeat,

    TimerExpired,

    LogRequest {
        leader_id: usize,
        current_term: usize,
        prefix_len: usize,
        prefix_term: usize,
        commit_length: usize,
        suffix: Vec<LogEntry>,
    },

    LogResponse {
        responder_id: usize,
        responder_term: usize,
        ack: usize,
        answer: bool,
    },
}
