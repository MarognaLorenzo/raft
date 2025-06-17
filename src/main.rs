mod component;
use component::{Component, Leader, Follower, Candidate, Initial};
use component::message::Message;
use tokio::task::JoinHandle;
use std::collections::HashMap;
use tokio::sync::mpsc::{self, Sender};
use std::{str, thread, usize, vec};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let n_servers = 5usize;
    let servers = initialize_servers(n_servers);
    let servers :Vec<Component<Candidate>>= servers.into_iter().map(|ser| ser.activate()).collect();
    println!("Server 0 amount of neighbours: {}", servers[0].neighbours_len());


    let handles:Vec<JoinHandle<_>> = vec![];
    for mut server in servers {
        let handle = tokio::spawn(async move {
            let m : Message = Message::VoteRequest{ candidate_id: server.get_name(), candidate_term: 0, log_length: 12, last_term: 11 };
            server.send_message(m, (server.get_name()+1)%n_servers);
            let message = server.open_message().show();
            println!("I ({}) received a message! {}",server.get_name(), message);
        });
        handles.push(handle);
        // TODO - Fix the mutable problems and understand why recv wants &mut self 
        // TODO - :
        // * Use the beam thing
        // * Get receiver from the world and receiver from private network
        // * Do a select loop in which you see which receiver has messages and then handle the
        // message 
        // * Set up a return mechanism in the loop for handling the message so that we can exit
        // from this thing.

    }

    println!("DONE");

    for handle in handles {
        handle.await.unwrap();
    }

}



pub fn initialize_servers(n_servers: usize) -> Vec<Component<Initial>> {
    let mut senders: Vec<Sender<Message>> = Vec::with_capacity(n_servers);

    let names: Vec<usize> = (0usize..n_servers).collect();
    let mut servers:Vec<Component<Initial>>= names.iter()
        .map(|&name| {
            let (sender, receiver) = mpsc::channel::<Message>(32);
            senders.push(sender);
            Component::<Initial>::new(
                name,
                n_servers,
                receiver,
                HashMap::<usize, Sender<Message>>::new()
                )
        })
    .collect();

    println!("Servers {}", servers.len());

    for (i, sender) in senders.iter().enumerate() {
        for (j, server) in servers.iter_mut().enumerate() {
            if i != j {
                let send= sender.clone();
                 server.add_sender(i, send);
            }
        }
    }

    
    return servers;
}
