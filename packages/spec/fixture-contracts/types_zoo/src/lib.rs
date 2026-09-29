#![no_std]

//! Fixture contract exercising every spec shape the reader must handle.
//!
//! Each function exists only so its signature lands in the `contractspecv0`
//! custom section: integers, bytes, options, results, collections, a tuple, a
//! struct nested three levels deep, unit and tuple enums (the spec union
//! arm), plus timepoint/duration primitives. Bodies are trivial.

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, Address, Bytes, BytesN, Duration, Env,
    Map, String, Symbol, Timepoint, Vec, I256, U256,
};

/// Third nesting level.
#[contracttype]
#[derive(Clone)]
pub struct Level3 {
    /// A plain integer leaf.
    pub value: u32,
}

/// Second nesting level.
#[contracttype]
#[derive(Clone)]
pub struct Level2 {
    /// The level below.
    pub inner: Level3,
}

/// Top of a three-level nesting chain.
#[contracttype]
#[derive(Clone)]
pub struct Level1 {
    /// The level below.
    pub inner: Level2,
}

/// Unit-only variants: emitted as a spec enum.
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    /// The color red.
    Red = 1,
    /// The color green.
    Green = 2,
}

/// Tuple variants: emitted as a spec union.
#[contracttype]
#[derive(Clone)]
pub enum Shape {
    /// A point in the plane.
    Point(u32, u32),
    /// Nothing at all.
    Empty,
}

/// The error set for the fallible demo below.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ZooError {
    /// The input was out of range.
    OutOfRange = 1,
}

#[contract]
pub struct TypesZoo;

#[contractimpl]
impl TypesZoo {
    /// Echoes every integer width the spec supports.
    pub fn ints(
        _env: Env,
        a: u32,
        b: i32,
        c: u64,
        d: i64,
        e: u128,
        f: i128,
        g: U256,
        h: I256,
    ) -> u32 {
        let _ = (b, c, d, e, f, g, h);
        a
    }

    /// Echoes a boolean.
    pub fn flip(_env: Env, v: bool) -> bool {
        !v
    }

    /// Echoes text and binary primitives.
    pub fn blobs(_env: Env, s: String, sym: Symbol, b: Bytes, fixed: BytesN<32>) -> Bytes {
        let _ = (s, sym, fixed);
        b
    }

    /// Echoes an account address.
    pub fn who(_env: Env, who: Address) -> Address {
        who
    }

    /// Echoes ledger-time primitives.
    pub fn when(_env: Env, at: Timepoint, lasts: Duration) -> Timepoint {
        let _ = lasts;
        at
    }

    /// Echoes an optional value.
    pub fn maybe(_env: Env, v: Option<u32>) -> Option<u32> {
        v
    }

    /// Echoes a fallible value.
    pub fn fallible(_env: Env, v: Result<u32, ZooError>) -> Result<u32, ZooError> {
        v
    }

    /// Echoes a vector.
    pub fn many(_env: Env, v: Vec<u32>) -> Vec<u32> {
        v
    }

    /// Echoes a map.
    pub fn lookup(_env: Env, m: Map<Symbol, u32>) -> Map<Symbol, u32> {
        m
    }

    /// Echoes a heterogeneous pair.
    pub fn pair(_env: Env, p: (u32, Symbol)) -> (u32, Symbol) {
        p
    }

    /// Echoes the three-level struct.
    pub fn nested(_env: Env, v: Level1) -> Level1 {
        v
    }

    /// Echoes the unit enum.
    pub fn paint(_env: Env, c: Color) -> Color {
        c
    }

    /// Echoes the tuple enum.
    pub fn shape(_env: Env, s: Shape) -> Shape {
        s
    }

    /// Returns nothing: the void output arm.
    pub fn noop(_env: Env) {}
}
