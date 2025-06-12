use super::{Component, Follower};
impl <MessageType> Component<Follower,MessageType> {
    pub fn lament (self) {
        println!("Oh no I am broken forever");
    }

}
