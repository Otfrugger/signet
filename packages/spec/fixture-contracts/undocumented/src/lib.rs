#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol, Vec};

#[contract]
pub struct Undocumented;

#[contractimpl]
impl Undocumented {
    pub fn ping(_env: Env) -> Symbol {
        symbol_short!("pong")
    }

    pub fn add(_env: Env, a: u32, b: u32) -> u32 {
        a + b
    }

    pub fn greet(_env: Env, who: Address, names: Vec<Symbol>) -> u32 {
        let _ = who;
        names.len()
    }
}
