mod component;
use component::message::ServerMessage;
use component::*;
use crossbeam::channel::*;
use std::collections::HashMap;
use std::io::{self, Write};
use std::time::Duration;
use std::{thread, usize};

use crate::component::order::Order;

fn main() {
    let n_servers = 5usize;
    let (servers, controllers) = initialize_servers(n_servers);

    let servers: Vec<Server<Candidate>> = servers.into_iter().map(|ser| ser.completed()).collect();
    // println!("Server 0 amount of neighbours: {}", servers[0].neighbours_len());

    let handles: Vec<_> = servers
        .into_iter()
        .map(|server| {
            std::thread::spawn(move || {
                let from = server.get_name();
                let to = (from + 1) % n_servers;
                let builded_message: ServerMessage = ServerMessage::Ping { from: from, to: to };
                if let Err(e) = server.send_message(builded_message, to) {
                    println!("Failed to send: {:?}", e.0);
                }
                let received_message = server.open_message();
                println!(
                    "I ({}) received a message! {:?}",
                    server.get_name(),
                    received_message
                );
                server.activate();
            })
        })
        .collect();

    thread::sleep(Duration::from_secs(3));

    println!();
    println!("Sending INFO!");

    controllers
        .iter()
        .for_each(|tx| tx.send(Order::SendInfo { info: 10 }).unwrap());

    thread::sleep(Duration::from_secs(3));

    println!();
    println!("Starting changing states");

    controllers.iter().enumerate().for_each(|(i, tx)| {
        let order: Order;
        if i % 2 == 0 {
            order = Order::ConvertToFollower;
        } else {
            order = Order::SendInfo { info: i }
        }
        tx.send(order).unwrap();
    });

    thread::sleep(Duration::from_secs(3));

    println!();
    println!("Exiting from everyone!");
    wait_for_user();
    controllers
        .iter()
        .for_each(|tx| tx.send(Order::Exit).unwrap());

    // TODO - :
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

pub fn initialize_servers(n_servers: usize) -> (Vec<Server<Initial>>, Vec<Sender<Order>>) {
    let mut senders: Vec<Sender<ServerMessage>> = Vec::with_capacity(n_servers);

    let mut controllers: Vec<Sender<Order>> = Vec::with_capacity(n_servers);
    let names: Vec<usize> = (0usize..n_servers).collect();
    let mut servers: Vec<Server<Initial>> = names
        .iter()
        .map(|&name| {
            let (server_message_sender, server_message_receiver) = crossbeam::channel::unbounded();
            let (controller, controller_receiver) = unbounded();
            senders.push(server_message_sender);
            controllers.push(controller);
            Server::<Initial>::new(
                name,
                n_servers,
                controller_receiver,
                server_message_receiver,
                HashMap::<usize, Sender<ServerMessage>>::new(),
            )
        })
        .collect();

    println!("Servers {}", servers.len());

    for (i, sender) in senders.iter().enumerate() {
        for (j, server) in servers.iter_mut().enumerate() {
            let send = sender.clone();
            server.add_sender(i, send);
        }
    }
    println!("Neighbours: {:?}", servers[0].neighbours);
    println!("Senders size: {}", servers[0].neighbours_len());

    return (servers, controllers);
}

fn wait_for_user() {
    print!("Press Enter to proceed...");
    io::stdout().flush().unwrap(); // Make sure the prompt is printed immediately
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
