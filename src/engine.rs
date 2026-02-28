use crate::pokemon::{Pokemon, PokemonType};
use crate::moves::MoveEffect;
use crate::types::Status;

pub struct BattleEngine {
    pub player: Pokemon,
    pub opponent: Pokemon,
}

impl BattleEngine {
    pub fn new(player: Pokemon, opponent: Pokemon) -> Self {
        Self { player, opponent }
    }

    pub fn get_type_multiplier(attacker_type: &PokemonType, defender_type: &PokemonType) -> f32 {
        match (attacker_type, defender_type) {
            (PokemonType::Fire, PokemonType::Grass) => 2.0,
            (PokemonType::Grass, PokemonType::Fire) => 0.5,
            (PokemonType::Water, PokemonType::Fire) => 2.0,
            (PokemonType::Fire, PokemonType::Water) => 0.5,
            _ => 1.0, // for each case not listed, the multiplier is 1.0
        }
    }

    pub fn execute_move(attacker: &mut Pokemon, defender: &mut Pokemon, move_index: usize) {
        if attacker.is_fainted() || defender.is_fainted() {
            return;
        }

        // clone the move so we don't hold a borrow on attacker while we also need to mutate defender
        let used_move = attacker.moves[move_index].clone();
        println!("\n> {} used {}!", attacker.name, used_move.name);

        for effect in &used_move.effects { //loop over effects
            match effect {
                MoveEffect::Damage => {
                    let type_multiplier = Self::get_type_multiplier(&used_move.move_type, &defender.primary_type);
                    let stab_multiplier = if used_move.move_type == attacker.primary_type {
                        1.5
                    } else {
                        1.0
                    };
                    let damage = (used_move.power as f32 * type_multiplier * stab_multiplier) as u32;

                    if type_multiplier > 1.0 { println!("It's super effective!"); }
                    else if type_multiplier < 1.0 { println!("It's not very effective..."); }
                    if stab_multiplier > 1.0 { println!("*STAB bonus applied!*"); }

                    defender.take_damage(damage);
                    println!("> {} took {} damage!", defender.name, damage);
                }

                MoveEffect::ApplyStatus { status, chance } => {
                    let roll: f32 = rand::random();

                    if roll < *chance {
                        if defender.status.is_none() {
                            println!("> {} was afflicted with {:?}!", defender.name, status);
                            defender.status = Some(status.clone());
                        } else {
                            println!("> But it failed — {} already has a status condition!", defender.name);
                        }
                    }
                }

                MoveEffect::ModifyStat { .. } => {
                    println!("> Stat modification not yet implemented.");
                }
                MoveEffect::Heal { .. } => {
                    println!("> Healing not yet implemented.");
                }
            }
        }
    }
}