# ferroDB
### A DHT-based distributed key-value store powered by libp2p.

### Usage
1. Spin up two or more terminal windows and run the binary in each of them with the following command. Each terminal acts as peer on the network and discover each other using mDNS. passive discovery.
```shell
cargo run
```

2. Adding records can be done by typing the following into the terminal once the application has started.
```shell
SET <key> <value>
```

3. Retrieving records can be done by typing the following.
```shell
GET <key>
```

Note: terminals can be shut down and restarted and the data will remain intact as long atleast a single peer remains alive. 

### Future Development
- Peer Discovery using the IPFS Kademlia DHT
- Record store instead of a Memory store for data persistence