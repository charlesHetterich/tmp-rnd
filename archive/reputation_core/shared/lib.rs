#![cfg_attr(not(feature = "std"), no_std)]

use ink::prelude::vec::Vec;
use ink::Address;

pub type CardId = u32;
pub type GameId = u32;

#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout))]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum CardType {
    Unit,
    Spell,
}

#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout))]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum EffectType {
    None = 0,
    Taunt = 1,
    Charge = 2,
    HealSelf = 3,
    DamageFront = 4,
}

#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout))]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub struct CardMetadata {
    pub id: CardId,
    pub name_hash: u32,
    pub rarity: u8,
    pub card_type: CardType,
    pub cost: u8,
    pub attack: u8,
    pub health: u8,
    pub effects: EffectType,
}

/// Interface for game contracts to query card data from NFT contracts
#[ink::trait_definition]
pub trait CardDataProvider {
    /// Get card metadata by ID
    #[ink(message)]
    fn get_card_metadata(&self, card_id: CardId) -> Option<CardMetadata>;
    
    /// Check if a card exists
    #[ink(message)]
    fn card_exists(&self, card_id: CardId) -> bool;
    
    /// Get all cards owned by an address
    #[ink(message)]
    fn cards_of_owner(&self, owner: Address) -> Vec<CardId>;
}

/// Game-specific types (could be moved to a separate game_types crate if needed)
#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout))]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub struct UnitInstance {
    pub card_id: CardId,
    pub current_hp: i16,
    pub acted_this_turn: bool,
}

#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout))]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub struct PlayerState {
    pub addr: Address,
    pub hp: i16,
    pub energy: u8,
    pub max_energy: u8,
    pub deck: Vec<CardId>,
    pub hand: Vec<CardId>,
    pub board: [Option<UnitInstance>; 4],
}

#[derive(PartialEq, Eq, Clone)]
#[cfg_attr(feature = "std", derive(Debug, ink::storage::traits::StorageLayout))]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum GameStatus {
    WaitingForPlayers,
    InProgress,
    Finished,
}

#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout))]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub struct Game {
    pub id: GameId,
    pub players: [PlayerState; 2],
    pub active_idx: u8,
    pub turn: u32,
    pub status: GameStatus,
}

#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout))]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum ActionType {
    PlayCard { hand_index: u8, slot_index: u8 },
    UseSpell { hand_index: u8, target_slot: u8 },
    EndTurn,
    Concede,
}