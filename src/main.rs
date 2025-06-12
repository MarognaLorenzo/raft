mod component;
use component::{Component, Leader, Follower, Candidate, Initial};
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{str, thread, usize, vec};

use crate::component::ComponentState;

fn main() {
    let n_servers = 5usize;
    let names: Vec<usize> = (0usize..n_servers).collect();

    let mut senders: Vec<Sender<i32>> = Vec::with_capacity(n_servers);

    let mut servers:Vec<Component<Initial, i32>>= names.iter().enumerate()
        .map(|(i, &name)| {
            let (sender, receiver) = channel::<i32>();
            senders.push(sender);
            Component::<Initial, i32>::new(name, n_servers, receiver, HashMap::<usize, Sender<i32>>::new() )
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

    let servers :Vec<Component<Candidate, i32>>= servers.into_iter().map(|ser| ser.activate()).collect();
    println!("Server 0 amount of neighbours: {}", servers[0].neighbours_len());
    servers[0].send_message(48392, 3);
    println!("Server received this: {}",servers[3].open_message());
}

