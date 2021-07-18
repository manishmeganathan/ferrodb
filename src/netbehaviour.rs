// This module implements functionality for a p2p network behaviour

use libp2p::NetworkBehaviour;
use libp2p::mdns::{Mdns, MdnsEvent};
use libp2p::kad::record::store::MemoryStore;
use libp2p::swarm::NetworkBehaviourEventProcess;
use libp2p::kad::{
    Kademlia
};

// A structure that represents a custom network behaviour 
// that combines Kademlia DHT and mDNS events.
#[derive(NetworkBehaviour)]
pub struct Behaviour {
    kademlia: Kademlia<MemoryStore>,
    mdns: Mdns
}

impl NetworkBehaviourEventProcess<MdnsEvent> for Behaviour {
    // Callback for when an mDNS event is produced
    fn inject_event(&mut self, event: MdnsEvent) {
        // If nodes are discovered through mDNS
        if let MdnsEvent::Discovered(list) = event {
            // Iterate over the list of discovered node peer IDs
            for (peer_id, multiaddr) in list {
                // Add the peer ID to the Kademlia DHT
                self.kademlia.add_address(&peer_id, multiaddr);
            }
        }
    }
}