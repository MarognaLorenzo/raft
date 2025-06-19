pub enum Order {
    SendInfo { info: usize },
    ConvertToFollower,
    ConvertToCandidate,
    Exit,
}
