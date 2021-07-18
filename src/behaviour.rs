// This module implements functionality for a p2p network behaviour

use std::str::from_utf8;
use libp2p::NetworkBehaviour;
use libp2p::mdns::{Mdns, MdnsEvent};
use libp2p::kad::record::store::MemoryStore;
use libp2p::swarm::NetworkBehaviourEventProcess;
use libp2p::kad::{
    Kademlia,
    KademliaEvent,
    QueryResult,
    Record,
    PeerRecord,
    PutRecordOk,
};

// A structure that represents a custom network behaviour 
// that combines Kademlia DHT and mDNS events.
#[derive(NetworkBehaviour)]
pub struct Behaviour {
    pub kademlia: Kademlia<MemoryStore>,
    pub mdns: Mdns
}

// Implementation of the mDNS event processor
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

// Implementation of the Kademlia event processor
impl NetworkBehaviourEventProcess<KademliaEvent> for Behaviour {
    // Callback for when a Kademlia event is produced
    fn inject_event(&mut self, event: KademliaEvent) {
        // Check the type of event
        match event {
            // If an outbound query has successfully completed, check type of query result
            KademliaEvent::OutboundQueryCompleted { result, .. } => match result {
                // If the GetRecord Query was successful
                QueryResult::GetRecord(Ok(ok)) => {
                    for PeerRecord { record: Record { key, value, .. }, .. } in ok.records {
                        println!("{:?} - {:?}", from_utf8(key.as_ref()).unwrap(), from_utf8(&value).unwrap());
                    }
                }
                // If the GetRecord Query was unsuccessful
                QueryResult::GetRecord(Err(err)) => {
                    eprintln!("Failed to retrieve a record for the key: {:?}", err);
                }

                // If the PutRecord Query was successful
                QueryResult::PutRecord(Ok(PutRecordOk { key })) => {
                    println!("Added record with key: {:?}", from_utf8(key.as_ref()).unwrap());
                }
                // If the PutRecord Query was unsuccessful
                QueryResult::PutRecord(Err(err)) => {
                    eprintln!("Failed to add record with the key: {:?}", err);
                }
                
                // All other query results are ignored
                _ => {}
            }

            _ => {}
        }
    }
}