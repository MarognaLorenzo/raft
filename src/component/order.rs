#[derive(Debug, Clone)]
pub enum Order {
    SendInfo { info: usize },
    ConvertToFollower,
    ConvertToCandidate,
    Disconnect,
    Reconnect,
    Exit,
}
