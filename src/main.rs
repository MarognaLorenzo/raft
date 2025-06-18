mod component;
use component::{Component, Leader, Follower, Candidate, Initial};
use component::message::Message;
use std::collections::HashMap;
use std::{str, thread, usize, vec};
use std::io::{self, Write};
use crossbeam::channel::*;

fn main() {
    let n_servers = 5usize;
    let servers = initialize_servers(n_servers);

    let servers :Vec<Component<Candidate>>= servers.into_iter().map(|ser| ser.activate()).collect();
    // println!("Server 0 amount of neighbours: {}", servers[0].neighbours_len());

    
    let handles:Vec<_> = servers.into_iter().map(
        |server| {
            std::thread::spawn( move || {
                let from = server.get_name();
                let to = (from + 1) % n_servers;
                let builded_message : Message = Message::Ping{ 
                    from: from, 
                    to: to,
                };
                if let Err(e) = server.send_message(builded_message, to) {
                    println!("Failed to send: {:?}", e.0);
                }
                let received_message = server.open_message(); 
                println!("I ({}) received a message! {:?}",server.get_name(), received_message);
            })
        }).collect();
    // TODO - Fix the mutable problems and understand why recv wants &mut self 
    // TODO - :
    // * Use the beam thing
    // * Get receiver from the world and receiver from private network
    // * Do a select loop in which you see which receiver has messages and then handle the
    // message 
    // * Set up a return mechanism in the loop for handling the message so that we can exit
    // from this thing.

    for handle in handles {
        handle.join().unwrap();
    }

    println!("DONE");

}



pub fn initialize_servers(n_servers: usize) -> Vec<Component<Initial>> {
    let mut senders: Vec<Sender<Message>> = Vec::with_capacity(n_servers);

    let names: Vec<usize> = (0usize..n_servers).collect();
    let mut servers:Vec<Component<Initial>>= names.iter()
        .map(|&name| {
            let (sender, receiver) = crossbeam::channel::unbounded();
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
    println!("Neighbours: {:?}", servers[0].neighbours);
    println!("Senders size: {}", servers[0].neighbours_len());

    
    return servers;
}
