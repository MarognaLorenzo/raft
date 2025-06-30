mod component;
use component::message::ServerMessage;
use component::*;
use crossbeam::channel::*;
use std::collections::HashMap;
use std::io::{self, Write};
use std::time::Duration;
use std::{thread, usize};

use crate::component::order::Order;
use std::env;

fn main() {
    env::set_var("RUST_LOG", "info");

    env_logger::init();

    // Example log call (can be from any thread)
    log::info!("Application started");

    let n_servers = 5usize;
    let (servers, controllers) = initialize_servers(n_servers);

    let servers: Vec<Server<Follower>> = servers.into_iter().map(|ser| ser.completed()).collect();
    // log::info!("Server 0 amount of neighbours: {}", servers[0].neighbours_len());

    let _handles: Vec<_> = servers
        .into_iter()
        .map(|server| {
            let mut thread_name = "Server-".to_string();
            thread_name.push_str(server.get_name().to_string().as_str());
            thread::Builder::new().name(thread_name).spawn(move || {
                let from = server.get_name();
                let to = (from + 1) % n_servers;
                let builded_message: ServerMessage = ServerMessage::Ping { from: from, to: to };
                if let Err(e) = server.send_message(builded_message, to) {
                    log::error!("Failed to send: {:?}", e.0);
                }
                let received_message = server.open_message();
                log::debug!(
                    "I ({}) received a message! {:?}",
                    server.get_name(),
                    received_message
                );
                if server.get_name() == 0 {
                    server
                        .get_self_sender()
                        .send(ServerMessage::TimerExpired)
                        .unwrap();
                }
                server.activate();
            })
        })
        .collect();

    // todo!("Add delays and disconnections from Servers");main
    wait_for_user();
    controllers.iter().for_each(|tx| tx.send(Order::Disconnect).unwrap());


    wait_for_user();
    controllers.iter().for_each(|tx| tx.send(Order::Reconnect).unwrap());

    wait_for_user();

    log::info!("\nSending INFO!");

    controllers
        .iter()
        .for_each(|tx| tx.send(Order::SendInfo { info: "Hello myself".to_string() }).unwrap());

    thread::sleep(Duration::from_secs(3));

    // log::info!();
    // log::info!("Starting changing states");
    //
    // controllers.iter().enumerate().for_each(|(i, tx)| {
    //     let order: Order;
    //     if i % 2 == 0 {
    //         order = Order::ConvertToFollower;
    //     } else {
    //         order = Order::SendInfo { info: i }
    //     }
    //     tx.send(order).unwrap();
    // });

    thread::sleep(Duration::from_secs(3));

    log::info!("\nExiting from everyone!");
    wait_for_user();
    controllers
        .iter()
        .for_each(|tx| tx.send(Order::Exit).unwrap());

    log::info!("DONE");
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
            senders.push(server_message_sender.clone());
            controllers.push(controller);
            Server::<Initial>::new(
                name,
                n_servers,
                controller_receiver,
                server_message_receiver,
                server_message_sender,
                HashMap::<usize, Sender<ServerMessage>>::new(),
            )
        })
        .collect();

    log::debug!("Servers {}", servers.len());

    for (i, sender) in senders.iter().enumerate() {
        for (j, server) in servers.iter_mut().enumerate() {
            if i!=j {
                server.add_sender(i, sender.clone());
            }
        }
    }
    log::debug!("Neighbours: {:?}", servers[0].neighbours);
    log::debug!("Senders size: {}", servers[0].neighbours_len());

    return (servers, controllers);
}

fn wait_for_user() {
    log::info!("\nPress Enter to proceed...\n");
    io::stdout().flush().unwrap(); // Make sure the prompt is printed immediately
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
