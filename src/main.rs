mod server;
use server::structures::message::ServerMessage;
use server::*;
use crossbeam::channel::*;
use std::collections::HashMap;
use std::io::{self, Write};
use std::{thread, usize};

use crate::server::structures::order::Order;
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
                    log::error!("Failed to send: {:?}", e);
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


    println!("Command Parser");
    println!("Enter commands in the format: <command> <server_index>");
    println!("Type 'exit' to quit the program.");
    println!("Type 'ls' to list server content");
    println!("Type 'dis' to disconnect a server");
    println!("Type 'con' to reconnect a server");
    println!("Type 's_<message>' to send a message to a server");

    // The main loop for processing commands
    loop {
        // --- 1. Get user input ---
        print!("> ");
        // Flush the output to ensure the prompt is displayed immediately
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new(); // Create a new, mutable String to store user input
        
        // Read a line from standard input and store it in the `input` string
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim whitespace from the input string and convert it to lowercase for case-insensitive comparison
        let trimmed_input = input.trim();

        // --- 2. Check for exit command ---
        if trimmed_input.to_lowercase() == "exit" {
            println!("Exiting the program. Goodbye!");
            break; // Exit the loop
        }

        // --- 3. Parse the command ---
        // Split the input string by whitespace to get individual parts
        let parts: Vec<&str> = trimmed_input.split_whitespace().collect();

        // Check if the command has exactly two parts (a number and a word)
        if parts.len() != 2 {
            println!("Invalid command format. Please use: <word> <number>");
            continue; // Go to the next iteration of the loop
        }

        // Attempt to parse the first part as a number (i32)
        let number_result = parts[1].parse::<usize>();
        let word = parts[0]; // The second part is the word

        // --- 4. Process the command ---
        match number_result {
            // If the first part was successfully parsed as a number
            Ok(number) => {
                // Now you have the number and the word, and you can perform actions based on them.
                println!("  Number: {}", number);
                println!("  Word:   '{}'", word);

                if number >= n_servers {
                    println!("  Error: Received number {} and server available are 0-{}", number, n_servers-1);
                    continue;
                }
                
                // You can add more logic here based on the 'word' as well.
                let lower_case_word = word.to_lowercase();
                if lower_case_word.starts_with("s_") {
                    if let Some(extracted_string) = lower_case_word.strip_prefix("s_") {
                        if !extracted_string.is_empty() {
                            println!("  Detected 's_' command! Extracted string: '{}'", extracted_string);
                            controllers[number]
                                .send(Order::SendInfo { info: extracted_string.to_string() })
                                .unwrap();
                        } else {
                            println!("  's_' command detected, but no string followed (e.g., 'send_').");
                        }
                    }
                }
                match lower_case_word.as_str() {
                    "dis" => {
                        println!("  Disconnecting {}", number);
                        controllers[number].send(Order::Disconnect).unwrap();
                    },
                    "con" => {
                        println!("  Connecting {}", number);
                        controllers[number].send(Order::Reconnect).unwrap();
                    },

                    "ls" => {

                        println!("  Listing {}", number);
                        controllers[number].send(Order::ListLog).unwrap();
                    }
                    "rust" => println!("  This is a Rust command!"),
                    _ => println!("  The word is not a valid command"),
                }
            }
            // If parsing the number failed (e.g., the first part was not a valid number)
            Err(e) => {
                println!("Error: Could not parse '{}' as a number. Details: {}", parts[0], e);
                // The loop continues, prompting for a new command
            }
        }
        println!("----------------------------------"); // Separator for clarity
    }
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

    return (servers, controllers);
}
