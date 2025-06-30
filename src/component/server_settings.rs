#[derive(Debug, Clone)]
pub struct ServerSettings{
    pub activated: bool,
    pub election_timeout: usize,
    pub heartbeat_timeout: usize,
    pub total_elements: usize,
}

impl ServerSettings {
    pub fn new(total_elements: usize) -> ServerSettings {
        ServerSettings {
            activated: true,
            election_timeout: 10,
            heartbeat_timeout: 2, 
            total_elements: total_elements,
        }
    }
}
