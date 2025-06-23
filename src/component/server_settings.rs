#[derive(Debug, Clone)]
pub struct ServerSettings{
    pub activated: bool,
    pub election_timeout: usize,
    pub heartbeat_timeout: usize,
}

impl ServerSettings {
    pub fn new() -> ServerSettings {
        ServerSettings {
            activated: true,
            election_timeout: 10,
            heartbeat_timeout: 2, 
        }
    }
}
