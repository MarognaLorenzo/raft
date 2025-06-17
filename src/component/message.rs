pub enum Message {
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
impl Message {
    pub fn show(&self) -> String { // Changed return type to String
        match self {
            Message::VoteRequest { candidate_id, candidate_term: _, log_length: _, last_term: _ } => {
                format!("{}", candidate_id) // Use format! for String conversion
            }
            Message::VoteResponse { responser_id: _, current_term: _, response } => {
                if *response { "yes".to_string() } else { "no".to_string() } // Convert &str to String
            }
        }
    }
}

