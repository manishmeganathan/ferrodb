// This module .. 

use libp2p::kad::{
    Kademlia,
    Quorum,
    Record,
};
use libp2p::kad::record::{
    Key,
    store::MemoryStore,
};

pub fn parse_input(kademlia: &mut Kademlia<MemoryStore>, line: String) {
    // Split input line by spaces
    let mut args = line.split(' ');

    // Check the first argument
    match args.next() {

        // If GET command 
        Some("GET") => {
            // Get the key
            let key = {
                // Retrieve the next argument
                match args.next() {
                    // Create a record Key if it exists
                    Some(key) => Key::new(&key),
                    // key was not provided
                    None => {
                        // Print error
                        eprintln!("Expected key");
                        return;
                    }
                }
            };

            // Retrieve the record from the memory store
            kademlia.get_record(&key, Quorum::One);
        }

        // If SET command
        Some("SET") => {
            // Get the key
            let key = {
                // Retrieve the next argument
                match args.next() {
                    // Create a record Key if it exists
                    Some(key) => Key::new(&key),
                    // key was not provided
                    None => {
                        // Print error
                        eprintln!("Expected key");
                        return;
                    }
                }
            };

            // Get the value
            let value = {
                // Retrieve the next argument
                match args.next() {
                    // Convert value to a vector of bytes if it exists
                    Some(value) => value.as_bytes().to_vec(),
                    // value was not provided
                    None => {
                        // Print error
                        eprintln!("Expected value");
                        return;
                    }
                }
            };

            // Create the key-value record
            let record = Record {key, value, publisher: None, expires: None};
            // Store the record on the memory store
            kademlia.put_record(record, Quorum::One).expect("Failed to store record locally.");
        }

        _ => {
            eprintln!("expected a GET or SET command.");
        }
    }
}