#![no_std]

use soroban_sdk::{contract, contracterror, contracttype};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    NotPurchased,
    Purchased,
    Redeemed,
}

#[contracttype]
enum DataKey {
    Holder,
    State,
}

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyPurchased = 2,
    AlreadyRedeemed = 3,
}

#[contract]
pub struct EventPass;
