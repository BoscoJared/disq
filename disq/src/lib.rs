//! The core library of disq, pronounced either "dis-queue" or "disk".
//! Disq is a message queueing system for both publishers and consumers.
//! While disq is implemented on-top of Discord, it is only necessary
//! to use Discord as the message broker and use this in an application
//! entirely external to the Discord bot world.
//!
//! Why Discord?
//!
//! - Discord is scalable, it has a lot of users!
//! - Discord has multi-region support, simply create a server in your target zone and voila!
//! - Discord is free.
//! - Discord handles persistence of messages by default, so is rather durable!
//! - Discord has great admin tooling such as search, timestamps, and even message replays!
//! - For fun!
//!
//! Why not Discord?
//!
//! - Discord does not have strong SLOs about availability since it is consumer-facing
//! - Discord has a global rate-limit of 50 requests / second / bot. Sharding is possible though :)
//! - Discord is a chat application
//!
//! Still here? Let's do this!

use serenity::client::ClientBuilder;

pub mod errors;
pub mod yeet;
pub mod yoink;

pub use crate::yeet::Yeeter;
pub use crate::yoink::Yoinker;

pub fn register(client_builder: ClientBuilder) -> ClientBuilder {
    client_builder
}
