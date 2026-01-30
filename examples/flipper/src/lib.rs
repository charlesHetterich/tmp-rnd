//! Flipper - A simple example PVM contract

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pvm::contract]
mod flipper {
    #[storage]
    pub struct Flipper {
        value: bool,
    }

    #[init]
    pub fn new() -> Flipper {
        Flipper { value: false }
    }

    #[call]
    pub fn flip(state: &mut Flipper) {
        state.value = !state.value;
    }

    #[call]
    pub fn get(state: &Flipper) -> bool {
        state.value
    }
}
