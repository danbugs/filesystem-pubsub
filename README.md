# `filesystem-pubsub`

This library can be used by adding the following to your `Cargo.toml`:
```toml
[dependencies]
filesystem-pubsub = { git = "https://github.com/danbugs/filesystem-pubsub" }
```

The library can then be used as follows:
```rust
use filesystem_pubsub::{Client, Message, Topic};

fn main() {
    let pubsub = Pubsub::open();

    let sub_tok = pubsub.subscribe("topic").unwrap();

    pubsub.publish("hello, world".as_bytes(), "topic").unwrap();

    let msg = pubsub.receive(sub_tok).unwrap();

    println!("received message: {:?}", std::str::from_utf8(msg).unwrap());
}