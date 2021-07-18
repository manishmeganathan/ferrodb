mod behaviour;
mod inputs;

use async_std::{io, task};
use std::error::Error;
use std::task::{Context, Poll};
use futures::prelude::*;

use libp2p::{identity, development_transport};
use libp2p::mdns::{Mdns, MdnsConfig};
use libp2p::kad::record::store::MemoryStore;
use libp2p::{
    PeerId, 
    Swarm,
    kad::Kademlia,
    swarm::SwarmEvent,
};

use crate::behaviour::Behaviour;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger
    env_logger::init();

    // Create a swarm to manage peers and events.
    let mut swarm = {
        // Create a random keypair from the secp256k1 curve.
        let keypair = identity::Keypair::generate_secp256k1();
        // Generate a PeerId from the public key.
        let peerid = PeerId::from(keypair.public());

        // Set up a standard P2P transport layer.
        let transport = development_transport(keypair).await?;

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

    // Listen on all interfaces and whatever port the OS assigns
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Setup buffered reader to read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    println!("Welcome to FerroDB!");

    // Start the swarm discovery and input loop
    task::block_on(future::poll_fn(move |cx: &mut Context<'_>| {
        loop {
            match stdin.try_poll_next_unpin(cx)? {        
                Poll::Ready(Some(line)) => inputs::parse_input(&mut swarm.behaviour_mut().kademlia, line),
                Poll::Ready(None) => panic!("Stdin closed"),
                Poll::Pending => break
            }
        }

        loop {
            match swarm.poll_next_unpin(cx) {
                Poll::Ready(Some(event)) => {
                    if let SwarmEvent::NewListenAddr { address, .. } = event {
                        println!("Listening on {:?}", address);
                    }
                }
                Poll::Ready(None) => return Poll::Ready(Ok(())),
                Poll::Pending => break,
            }
        }
        
        Poll::Pending
    }))
}