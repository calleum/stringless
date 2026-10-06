# stringless

Type-level Socket.IO protocol design for Rust: events as typed values.

One descriptor type per event, carrying direction, payload, and namespace
marker. 

No macros, no runtime registration. The wrong direction, payload or namespace
fail to compile.

```rust
use stringless::*;

struct Conversation;
impl Protocol for Conversation {
    type Auth = ();
    const NAMESPACE: &'static str = "/conversation";
}

struct Command;
struct QueuedPayload;

impl Conversation {
    const COMMAND: ClientEvent<Self, Command> = ClientEvent::new("command");
    const QUEUED: ServerEvent<Self, QueuedPayload> = ServerEvent::new("queued");
}

// These fail to compile:
//   socket.emit(Conversation::COMMAND, &command);  // wrong direction
//   socket.emit(Conversation::QUEUED, &other);     // wrong payload
//   socket.emit(Other::QUEUED, &queued);           // wrong namespace
```

Companion to [post](https://calleum.au/blog/type-safe-socketio-rust/).

The three compile-fail guarantees are tested in the crate docs (`cargo test`).
