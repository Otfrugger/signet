#![no_std]

//! Fixture contract emitting `#[contractevent]` types so the reader's event
//! entries have something to parse.

use soroban_sdk::{contract, contractevent, contractimpl, Address, Env, Symbol};

/// A transfer between two accounts.
#[contractevent]
#[derive(Clone)]
pub struct Transfer {
    /// The sender.
    #[topic]
    pub from: Address,
    /// The recipient.
    #[topic]
    pub to: Address,
    /// How much moved.
    pub amount: i128,
}

/// A checkpoint with no topics.
#[contractevent]
#[derive(Clone)]
pub struct Checkpoint {
    /// Monotonic sequence number.
    pub seq: u32,
    /// Human-readable tag.
    pub tag: Symbol,
}

#[contract]
pub struct Events;

#[contractimpl]
impl Events {
    /// Publishes a transfer event.
    pub fn send(env: Env, from: Address, to: Address, amount: i128) {
        Transfer { from, to, amount }.publish(&env);
    }

    /// Publishes a checkpoint event.
    pub fn check(env: Env, seq: u32, tag: Symbol) {
        Checkpoint { seq, tag }.publish(&env);
    }
}
