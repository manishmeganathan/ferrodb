// This module implements functionality for a p2p network swarm.

use libp2p::{
    identity,
    development_transport
};
use libp2p::PeerId;

pub async fn setup_swarm() {
    // Create a random keypair from the secp256k1 curve.
    let keypair = identity::Keypair::generate_secp256k1();
    // Generate a PeerId from the public key.
    let peerid = PeerId::from(keypair.public());

    // Set up a standard P2P transport layer.
    let transport = development_transport(keypair).await?;

}