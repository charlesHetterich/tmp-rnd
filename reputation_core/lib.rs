#![cfg_attr(not(feature = "std"), no_std, no_main)]

use shared::{
    CardId, GameId, CardMetadata, CardDataProvider,
    UnitInstance, PlayerState, GameStatus, Game, ActionType
};
use scale::{Encode, Decode};

#[ink::contract]
mod cards_on_chain {
    use super::*;
    use ink::prelude::{vec, vec::Vec};
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct CardsOnChain {
        // Reference to card NFT contract
        card_contract: Option<Address>,
        
        // Game storage
        games: Mapping<GameId, Game>,
        next_game_id: GameId,
        waiting_player: Option<Address>,
        player_active_game: Mapping<Address, GameId>,
    }


    #[ink(event)]
    pub struct GameStarted {
        #[ink(topic)]
        game_id: GameId,
        #[ink(topic)]
        player_a: Address,
        #[ink(topic)]
        player_b: Address,
    }

    #[ink(event)]
    pub struct ActionExecuted {
        #[ink(topic)]
        game_id: GameId,
        #[ink(topic)]
        player: Address,
        action: ActionType,
    }

    #[ink(event)]
    pub struct TurnEnded {
        #[ink(topic)]
        game_id: GameId,
        new_active_player: Address,
        turn: u32,
    }

    #[ink(event)]
    pub struct GameEnded {
        #[ink(topic)]
        game_id: GameId,
        #[ink(topic)]
        winner: Address,
    }

    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        GameNotFound,
        NotYourTurn,
        InvalidAction,
        NotEnoughEnergy,
        SlotOccupied,
        InvalidSlot,
        InvalidHandIndex,
        GameAlreadyFinished,
        AlreadyInGame,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl CardsOnChain {
        #[ink(constructor)]
        pub fn new(card_contract: Option<Address>) -> Self {
            Self {
                card_contract,
                games: Mapping::default(),
                next_game_id: 1,
                waiting_player: None,
                player_active_game: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn set_card_contract(&mut self, card_contract: Address) {
            self.card_contract = Some(card_contract);
        }

        fn get_card_data(&self, card_id: CardId) -> Option<CardMetadata> {
            if let Some(card_contract_addr) = self.card_contract {
                let card_contract: ink::contract_ref!(CardDataProvider) = card_contract_addr.into();
                card_contract.get_card_metadata(card_id)
            } else {
                // Fallback for when card contract is not set - use hardcoded data for testing
                use shared::{CardMetadata, CardType, EffectType};
                match card_id {
                    1 => Some(CardMetadata {
                        id: 1,
                        name_hash: 0x1234,
                        rarity: 1,
                        card_type: CardType::Unit,
                        cost: 1,
                        attack: 1,
                        health: 1,
                        effects: EffectType::None,
                    }),
                    2 => Some(CardMetadata {
                        id: 2,
                        name_hash: 0x5678,
                        rarity: 1,
                        card_type: CardType::Unit,
                        cost: 2,
                        attack: 2,
                        health: 2,
                        effects: EffectType::None,
                    }),
                    3 => Some(CardMetadata {
                        id: 3,
                        name_hash: 0x9abc,
                        rarity: 2,
                        card_type: CardType::Unit,
                        cost: 3,
                        attack: 3,
                        health: 3,
                        effects: EffectType::Charge,
                    }),
                    _ => None,
                }
            }
        }

        #[ink(message)]
        pub fn register_for_match(&mut self) -> Result<GameId> {
            let caller = self.env().caller();
            
            // Check if player is already in a game
            if self.player_active_game.contains(&caller) {
                return Err(Error::AlreadyInGame);
            }

            if let Some(waiting) = self.waiting_player {
                if waiting == caller {
                    return Err(Error::AlreadyInGame);
                }
                
                // Create new game
                let game_id = self.create_game(waiting, caller)?;
                self.waiting_player = None;
                Ok(game_id)
            } else {
                self.waiting_player = Some(caller);
                Ok(0) // Return 0 to indicate waiting
            }
        }

        fn create_game(&mut self, player_a: Address, player_b: Address) -> Result<GameId> {
            let game_id = self.next_game_id;
            
            // Create default deck for each player
            let default_deck = vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1]; // 10 cards for demo

            let mut game = Game {
                id: game_id,
                players: [
                    PlayerState {
                        addr: player_a,
                        hp: 20,
                        energy: 1,
                        max_energy: 1,
                        deck: default_deck.clone(),
                        hand: Vec::new(),
                        board: [None, None, None, None],
                    },
                    PlayerState {
                        addr: player_b,
                        hp: 20,
                        energy: 0,
                        max_energy: 1,
                        deck: default_deck,
                        hand: Vec::new(),
                        board: [None, None, None, None],
                    },
                ],
                active_idx: 0,
                turn: 1,
                status: GameStatus::InProgress,
            };

            // Draw initial hands (3 cards each)
            for player_idx in 0..2 {
                for _ in 0..3 {
                    self.draw_card(&mut game, player_idx);
                }
            }

            self.games.insert(game_id, &game);
            self.player_active_game.insert(&player_a, &game_id);
            self.player_active_game.insert(&player_b, &game_id);
            self.next_game_id += 1;

            self.env().emit_event(GameStarted {
                game_id,
                player_a,
                player_b,
            });

            Ok(game_id)
        }

        fn draw_card(&self, game: &mut Game, player_idx: usize) {
            if !game.players[player_idx].deck.is_empty() && game.players[player_idx].hand.len() < 7 {
                // Simple deterministic draw using block number
                let block_number = self.env().block_number();
                let draw_index = (block_number as usize + game.turn as usize) % game.players[player_idx].deck.len();
                let card_id = game.players[player_idx].deck.remove(draw_index);
                game.players[player_idx].hand.push(card_id);
            }
        }

        #[ink(message)]
        pub fn submit_turn_actions(&mut self, game_id: GameId, actions: Vec<ActionType>) -> Result<()> {
            let caller = self.env().caller();
            let mut game = self.games.get(game_id).ok_or(Error::GameNotFound)?;
            
            if game.status == GameStatus::Finished {
                return Err(Error::GameAlreadyFinished);
            }

            let active_player = &game.players[game.active_idx as usize];
            if active_player.addr != caller {
                return Err(Error::NotYourTurn);
            }

            // Execute actions in sequence
            for action in &actions {
                self.execute_action(&mut game, action.clone())?;
                
                self.env().emit_event(ActionExecuted {
                    game_id,
                    player: caller,
                    action: action.clone(),
                });
            }

            // Check if turn ended
            if actions.iter().any(|a| matches!(a, ActionType::EndTurn)) {
                self.end_turn(&mut game)?;
            }

            self.games.insert(game_id, &game);
            Ok(())
        }

        fn execute_action(&self, game: &mut Game, action: ActionType) -> Result<()> {
            let active_idx = game.active_idx as usize;
            
            match action {
                ActionType::PlayCard { hand_index, slot_index } => {
                    if hand_index as usize >= game.players[active_idx].hand.len() {
                        return Err(Error::InvalidHandIndex);
                    }
                    if slot_index >= 4 {
                        return Err(Error::InvalidSlot);
                    }
                    if game.players[active_idx].board[slot_index as usize].is_some() {
                        return Err(Error::SlotOccupied);
                    }

                    let card_id = game.players[active_idx].hand[hand_index as usize];
                    
                    // Get card metadata from card contract
                    let card_data = self.get_card_data(card_id).ok_or(Error::InvalidAction)?;
                    
                    if game.players[active_idx].energy < card_data.cost {
                        return Err(Error::NotEnoughEnergy);
                    }

                    // Play the card
                    game.players[active_idx].energy -= card_data.cost;
                    game.players[active_idx].hand.remove(hand_index as usize);

                    game.players[active_idx].board[slot_index as usize] = Some(UnitInstance {
                        card_id,
                        current_hp: card_data.health as i16,
                        acted_this_turn: false,
                    });
                },
                ActionType::UseSpell { hand_index: _, target_slot: _ } => {
                    // Spell implementation would go here
                    return Err(Error::InvalidAction);
                },
                ActionType::EndTurn => {
                    // This is handled in submit_turn_actions
                },
                ActionType::Concede => {
                    game.status = GameStatus::Finished;
                    let opponent_idx = 1 - active_idx;
                    
                    self.env().emit_event(GameEnded {
                        game_id: game.id,
                        winner: game.players[opponent_idx].addr,
                    });
                },
            }
            
            Ok(())
        }

        fn end_turn(&mut self, game: &mut Game) -> Result<()> {
            let active_idx = game.active_idx as usize;
            let opponent_idx = 1 - active_idx;

            // Combat phase: units attack forward
            // Collect damage to apply after calculating all attacks
            let mut damage_to_units: Vec<(usize, usize, i16)> = Vec::new(); // (player_idx, slot, damage)
            let mut damage_to_players: Vec<(usize, i16)> = Vec::new(); // (player_idx, damage)

            for slot in 0..4 {
                if let Some(attacker) = &game.players[active_idx].board[slot] {
                    let attack_power = match attacker.card_id {
                        1 => 1,
                        2 => 2,
                        3 => 3,
                        _ => 1,
                    };

                    if let Some(defender) = &game.players[opponent_idx].board[slot] {
                        // Unit vs unit combat
                        let defender_attack = match defender.card_id {
                            1 => 1,
                            2 => 2,
                            3 => 3,
                            _ => 1,
                        };

                        damage_to_units.push((active_idx, slot, defender_attack));
                        damage_to_units.push((opponent_idx, slot, attack_power));
                    } else {
                        // Attack player directly
                        damage_to_players.push((opponent_idx, attack_power));
                    }
                }
            }

            // Apply damage to units
            for (player_idx, slot, damage) in damage_to_units {
                if let Some(unit) = &mut game.players[player_idx].board[slot] {
                    unit.current_hp -= damage;
                }
            }

            // Apply damage to players
            for (player_idx, damage) in damage_to_players {
                game.players[player_idx].hp -= damage;
            }

            // Remove dead units
            for player_idx in 0..2 {
                for slot in 0..4 {
                    if let Some(ref unit) = game.players[player_idx].board[slot] {
                        if unit.current_hp <= 0 {
                            game.players[player_idx].board[slot] = None;
                        }
                    }
                }
            }

            // Check for game end
            if game.players[0].hp <= 0 {
                game.status = GameStatus::Finished;
                self.env().emit_event(GameEnded {
                    game_id: game.id,
                    winner: game.players[1].addr,
                });
                return Ok(());
            } else if game.players[1].hp <= 0 {
                game.status = GameStatus::Finished;
                self.env().emit_event(GameEnded {
                    game_id: game.id,
                    winner: game.players[0].addr,
                });
                return Ok(());
            }

            // Switch active player
            game.active_idx = 1 - game.active_idx;
            game.turn += 1;

            let new_active_idx = game.active_idx as usize;
            
            // Increase max energy and refresh energy
            if game.players[new_active_idx].max_energy < 10 {
                game.players[new_active_idx].max_energy += 1;
            }
            game.players[new_active_idx].energy = game.players[new_active_idx].max_energy;

            // Draw a card for the new active player at start of their turn
            self.draw_card(game, new_active_idx);
            
            // Draw a card for the opponent when we end our turn
            let previous_active_idx = 1 - new_active_idx;
            self.draw_card(game, previous_active_idx);

            // Reset unit action flags
            for unit in &mut game.players[new_active_idx].board {
                if let Some(u) = unit {
                    u.acted_this_turn = false;
                }
            }

            self.env().emit_event(TurnEnded {
                game_id: game.id,
                new_active_player: game.players[new_active_idx].addr,
                turn: game.turn,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn get_game_state(&self, game_id: GameId) -> Option<Game> {
            self.games.get(game_id)
        }

        #[ink(message)]
        pub fn get_player_game(&self, player: Address) -> Option<GameId> {
            self.player_active_game.get(player)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let contract = CardsOnChain::new(None);
            assert_eq!(contract.next_game_id, 1);
        }

        #[ink::test]
        fn get_card_data_works() {
            let contract = CardsOnChain::new(None);
            
            // Test hardcoded fallback data
            let card_data = contract.get_card_data(1).unwrap();
            assert_eq!(card_data.cost, 1);
            assert_eq!(card_data.attack, 1);
            assert_eq!(card_data.health, 1);
        }

        #[ink::test]
        fn matchmaking_works() {
            let mut contract = CardsOnChain::new(None);
            let alice = Address::from([0x1; 20]);
            let bob = Address::from([0x2; 20]);

            // Set the contract caller to alice for the first registration
            ink::env::test::set_caller(alice);
            let result1 = contract.register_for_match().unwrap();
            assert_eq!(result1, 0); // Alice is waiting

            // Set the contract caller to bob for the second registration
            ink::env::test::set_caller(bob);
            let game_id = contract.register_for_match().unwrap();
            assert_eq!(game_id, 1); // Game created

            let game = contract.get_game_state(game_id).unwrap();
            assert_eq!(game.players[0].addr, alice);
            assert_eq!(game.players[1].addr, bob);
            assert_eq!(game.status, GameStatus::InProgress);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_game_creation(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = CardsOnChainRef::new(None);
            let contract = client
                .instantiate("cards_on_chain", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<CardsOnChain>();

            // Test getting game state (should be none initially)
            let get_game = call_builder.get_game_state(1);
            let get_result = client.call(&ink_e2e::alice(), &get_game).dry_run().await?;
            assert!(get_result.return_value().is_none());

            Ok(())
        }

        #[ink_e2e::test]
        async fn e2e_full_game_flow(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = CardsOnChainRef::new(None);
            let contract = client
                .instantiate("cards_on_chain", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<CardsOnChain>();

            // Alice registers for match
            let register_alice = call_builder.register_for_match();
            let _result = client
                .call(&ink_e2e::alice(), &register_alice)
                .submit()
                .await
                .expect("alice register failed");

            // Bob registers for match (should create game)
            let register_bob = call_builder.register_for_match();
            let result = client
                .call(&ink_e2e::bob(), &register_bob)
                .submit()
                .await
                .expect("bob register failed");

            // Get game state
            let get_game = call_builder.get_game_state(1);
            let game_result = client.call(&ink_e2e::alice(), &get_game).dry_run().await?;
            assert!(game_result.return_value().is_some());

            Ok(())
        }
    }
}
