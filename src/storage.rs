struct SendStream {
    id: [u8; 16],

    // `send_keys` and `usernames` are ordered and correlated
    send_keys: Vec<[u8; 32]>,
    usernames: Vec<String>,
}

impl SendStream {
    pub fn put(&self) {}
}

struct RecvStream {
    id: [u8; 16],
    position: u64,
}

impl RecvStream {
    pub fn sync(&self) {}
}

/*
`Interaction` on-disk format

- receive streams count
    - size = 2 bits
    - count = 0-8 bytes
- receive stream * receive streams count
    - id = 16 bytes
    - position
        - size = 2 bits
        - position = 0-8 bytes

- send stream
    - id = 16 bytes
    - recipients count
        - size = 2 bits
        - count = 0-8 bytes
    - send_keys = 32 bytes * recipient count
    - usernames = variable bytes * recipient count (encrypted)

- recv_keys
    - start = 8 bytes
    - end = 8 bytes
    - pub key = 32 bytes
    - priv key = 32 bytes (encrypted)
*/
struct Interaction {
    id: [u8; 16],

    send_stream: SendStream,
    recv_streams: Vec<RecvStream>,

    // probably should be a BTreeMap but for now
    // we're just tracking (start, end, pub key, priv key) in a tuple
    recv_keys: Vec<(u64, u64, [u8; 32], [u8; 32])>,
}

impl Interaction {
    pub fn put(&self) {
        self.send_stream.put()
    }

    pub fn sync_all(&self) {
        for recv_stream in &self.recv_streams {
            recv_stream.sync()
        }
    }
}

// ?? will need a "bootstrapping stream" for adding a new user to an interaction
//  * sharing your public key for them to encrypt with
//  * sharing their position on your send stream
//  * sharing the current ratchet state
//  * sharing any "historical keys" for your stream

// ?? will also need "repair streams" maybe? but those will likely just be identical to
// ?? bootstrapping streams as they serve the same "re-sync" purpose.
