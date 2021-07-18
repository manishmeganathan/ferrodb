// This module implements functionality for a p2p network swarm.

use async_std::task;
use libp2p::{identity, development_transport};
use libp2p::mdns::{Mdns, MdnsConfig};
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{
    Kademlia
};
use libp2p::{
    PeerId, 
    Swarm
};

use crate::netbehaviour::Behaviour;

pub async fn setup_swarm() {
    // Create a random keypair from the secp256k1 curve.
    let keypair = identity::Keypair::generate_secp256k1();
    // Generate a PeerId from the public key.
    let peerid = PeerId::from(keypair.public());

    // Set up a standard P2P transport layer.
    let transport = development_transport(keypair).await?;

    // Create a swarm to manage peers and events.
    let mut swarm = {
        // Create a Memory Store
        let store = MemoryStore::new(peerid);
        // Setup the Memory Store with Kademlia 
        let kademlia = Kademlia::new(peerid, store);

        // Create the mDNS discovery service for the swarm
        let mdns = task::block_on(Mdns::new(MdnsConfig::default()))?;
        // Create the network behaviour for the swarm
        let behaviour = Behaviour { kademlia, mdns };

        // Create the swarm
        Swarm::new(transport, behaviour, peerid)
    };
}