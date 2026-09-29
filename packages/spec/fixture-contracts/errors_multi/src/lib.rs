#![no_std]

//! Fixture contract with two `#[contracterror]` enums so the reader sees more
//! than one error set in a single spec.

use soroban_sdk::{contract, contracterror, contractimpl, Env};

/// Failures from storing values.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum StoreError {
    /// Nothing was stored under the key.
    Missing = 1,
    /// The stored value failed validation.
    Invalid = 2,
}

/// Failures from access control.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AuthError {
    /// The caller is not allowed in.
    Denied = 1,
    /// The caller's session lapsed.
    Expired = 2,
}

#[contract]
pub struct ErrorsMulti;

#[contractimpl]
impl ErrorsMulti {
    /// Always fails while storing.
    pub fn store(_env: Env) -> Result<(), StoreError> {
        Err(StoreError::Missing)
    }

    /// Always fails while authorizing.
    pub fn enter(_env: Env) -> Result<(), AuthError> {
        Err(AuthError::Denied)
    }
}
