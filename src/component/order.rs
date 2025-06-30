#[derive(Debug, Clone)]
pub enum Order {
    SendInfo { info: String },
    ConvertToFollower,
    ConvertToCandidate,
    ListLog,
    Disconnect,
    Reconnect,
    Exit,
}
