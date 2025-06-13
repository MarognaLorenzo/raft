mod component;
use component::{Component, Leader, Follower, Candidate, Initial};
use component::message::Message;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{sleep, JoinHandle};
use std::{str, thread, usize, vec};
use std::io::{self, Write};

fn main() {
    let n_servers = 5usize;
    let servers = initialize_servers(n_servers);
    let servers :Vec<Component<Candidate>>= servers.into_iter().map(|ser| ser.activate()).collect();
    println!("Server 0 amount of neighbours: {}", servers[0].neighbours_len());


/* 
    let handles:Vec<JoinHandle<_>> = servers.into_iter()
        .map(|server| {
            thread::spawn(move || {
                // loop {
                    server.send_message(10000+server.get_name() as i32, (server.get_name()+1)%n_servers);
                    let message = server.open_message();
                    println!("I ({}) received a message! {}",server.get_name(), message );
                // }
            })
        })
    .collect();

    println!("DONE");

    for handle in handles {
        handle.join().unwrap();
    }
 */

    
}



pub fn initialize_servers(n_servers: usize) -> Vec<Component<Initial>> {
    let mut senders: Vec<Sender<Message>> = Vec::with_capacity(n_servers);

    let names: Vec<usize> = (0usize..n_servers).collect();
    let mut servers:Vec<Component<Initial>>= names.iter()
        .map(|&name| {
            let (sender, receiver) = channel::<Message>();
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
