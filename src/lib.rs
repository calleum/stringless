//! Type-level Socket.IO protocol design.
//!
//! One descriptor type per event, carrying direction, payload, and namespace
//! marker. No macros or runtime registration.
//!
//! The compile-fail examples below are the point of the design. Each is a
//! misuse that must not compile:
//!
//! Wrong direction (a client event cannot be emitted by the server):
//!
//! ```compile_fail
//! use stringless::*;
//! struct Conversation;
//! impl Protocol for Conversation { type Auth = (); const NAMESPACE: &'static str = "/conversation"; }
//! struct Command;
//! # fn main() {
//! let socket: TypedSocket<Conversation> = unimplemented!();
//! socket.emit(Conversation::COMMAND, &Command);
//! # }
//! ```
//!
//! Wrong payload (QUEUED carries QueuedPayload, not OtherPayload):
//!
//! ```compile_fail
//! use stringless::*;
//! struct Conversation;
//! impl Protocol for Conversation { type Auth = (); const NAMESPACE: &'static str = "/conversation"; }
//! struct QueuedPayload;
//! struct OtherPayload;
//! # fn main() {
//! let socket: TypedSocket<Conversation> = unimplemented!();
//! socket.emit(Conversation::QUEUED, &OtherPayload);
//! # }
//! ```
//!
//! Wrong namespace (an event from another protocol cannot pass through):
//!
//! ```compile_fail
//! use stringless::*;
//! struct Conversation;
//! impl Protocol for Conversation { type Auth = (); const NAMESPACE: &'static str = "/conversation"; }
//! struct Other;
//! impl Protocol for Other { type Auth = (); const NAMESPACE: &'static str = "/other"; }
//! struct QueuedPayload;
//! # fn main() {
//! let socket: TypedSocket<Conversation> = unimplemented!();
//! socket.emit(Other::QUEUED, &QueuedPayload);
//! # }
//! ```

use std::error::Error;
use std::fmt;
use std::marker::PhantomData;

/// A protocol is one Socket.IO namespace plus its auth payload.
pub trait Protocol {
    type Auth;
    const NAMESPACE: &'static str;
}

/// An event the client sends. You can route it, but not emit it from the server.
pub struct ClientEvent<P, T>(PhantomData<(P, T)>);
/// An event the server emits. You cannot pass it to the inbound router.
pub struct ServerEvent<P, T>(PhantomData<(P, T)>);
/// One event name used in both directions, with a different payload each way.
pub struct DuplexEvent<P, In, Out>(PhantomData<(P, In, Out)>);

macro_rules! event_new {
    ($ty:ident) => {
        impl<P: Protocol, T> $ty<P, T> {
            /// The argument is the wire name and the type carries the direction and payload.
            pub const fn new(_name: &'static str) -> Self {
                Self(PhantomData)
            }
        }
    };
}

event_new!(ClientEvent);
event_new!(ServerEvent);

impl<P: Protocol, In, Out> DuplexEvent<P, In, Out> {
    /// The argument is the wire name and the type carries both payloads.
    pub const fn new(_name: &'static str) -> Self {
        Self(PhantomData)
    }
}

/// A socket bound to one namespace. It only emits that namespace's
/// outbound events.
pub struct TypedSocket<P: Protocol>(PhantomData<P>);

#[derive(Debug)]
pub struct EmitError;

impl fmt::Display for EmitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("emit failed")
    }
}

impl Error for EmitError {}

impl<P: Protocol> TypedSocket<P> {
    /// Signature only. This sketch has no wire I/O, so it always errors.
    pub fn emit<T>(&self, _event: ServerEvent<P, T>, _payload: &T) -> Result<(), EmitError> {
        Err(EmitError)
    }
}

/// The full event inventory for one protocol, independent of any runtime
/// router.
pub struct EventCatalog<P: Protocol> {
    _marker: PhantomData<P>,
}

impl<P: Protocol> EventCatalog<P> {
    pub fn add<E>(&mut self, _event: &E) -> &mut Self {
        self
    }
}
