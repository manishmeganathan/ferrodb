// This module implements functionality for a p2p network behaviour

use libp2p::NetworkBehaviour;
use libp2p::mdns::{Mdns};
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{
    Kademlia
};

// A structure that represents a custom network behaviour 
// that combines Kademlia DHT and mDNS events.
#[derive(NetworkBehaviour)]
struct Behaviour {
    kademlia: Kademlia<MemoryStore>,
    mdns: Mdns
}
