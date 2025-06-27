#[derive(Debug, Clone)]
pub enum Order {
    SendInfo { info: String },
    ConvertToFollower,
    ConvertToCandidate,
    Disconnect,
    Reconnect,
    Exit,
}
