#[derive(Debug, Clone)]
pub struct ServerSettings{
    pub activated: bool,
}

impl ServerSettings {
    pub fn new() -> ServerSettings {
        ServerSettings {
            activated: true,
        }
    }
}
