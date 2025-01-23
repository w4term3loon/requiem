## Protocol design
### Payload
- Has information about the sender node
- Has information about the target node
- Has place for additional information about the network (self organization)
- Has place for crucial telemetry data that need to be communicated (vulnerability logs)
- Encompasses all these things while building a block

### Communication
- TCP/IP as communication layer
- Pub/sub as abstraction with topics
- Each node can host a topic as publisher
- Each node can subscribe to other topics
- Messages are serialized after encryption
- Decentralized architecture makes routing easier (does it tho?)
- Figure out message propagation scheme (gossip protocol)

### Problems
- Brokers need to handle requests async (thread pools or async io)
- Offline subscribers miss messages (message queue to buffer messages)
- Unauthorized parties can publish/subscribe (some authentication)
- Messages might get lost (?)
- Distributed problems:
    - Decentralized brokers need gossip protocols to share topics across nodes
    - Topics should be shared somehow across nodes (Napster?)

### Ideas
- Propagate forensic tombstones (attack data) across the swarm
- Distribute Shamir-shares for spore activation
- Detect compromised nodes and trigger CAP self-destructs

### Interface
### Pack
### Messages
### Unpack
