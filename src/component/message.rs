#[derive(Debug)]
pub enum ServerMessage {
    Ping{
        from: usize, 
        to: usize,
    },
    VoteRequest {
        candidate_id: usize,
        candidate_term: usize,
        log_length: usize,
        last_term: usize,
    },
    VoteResponse{
        responser_id: usize,
        current_term: usize,
        response: bool,
    }
}

