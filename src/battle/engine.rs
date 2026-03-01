use crate::pokemon::{Pokemon, PokemonType};
use crate::moves::{MoveEffect, Stat, StatTarget};
use crate::types::{Status, get_type_multiplier};

// handles all damage calculation, move execution, and status effect processing for individual pokemon actions
pub struct BattleEngine {
    pub player: Pokemon,
    pub opponent: Pokemon,
}

impl BattleEngine {
    pub fn new(player: Pokemon, opponent: Pokemon) -> Self {
        Self { player, opponent }
    }

    fn stage_to_multiplier(stage: i32) -> f32 {
        match stage {
            -6 => 0.25,
            -5 => 0.28,
            -4 => 0.33,
            -3 => 0.40,
            -2 => 0.50,
            -1 => 0.67,
             0 => 1.00,
             1 => 1.50,
             2 => 2.00,
             3 => 2.50,
             4 => 3.00,
             5 => 3.50,
             6 => 4.00,

            _ => 1.00,
        }
    }

    pub fn process_end_of_turn(pokemon: &mut Pokemon) {
        match &pokemon.status {
            Some(Status::Burn) => {
                let burn_damage = (pokemon.max_hp / 8).max(1); // .max(1) ensures burn always deals at least 1 damage
                println!("> {} is hurt by its burn!", pokemon.name);
                pokemon.take_damage(burn_damage);
            }
            _ => {} // for now, do nothing
        }
    }

    pub fn execute_move(attacker: &mut Pokemon, defender: &mut Pokemon, move_index: usize) {
        if attacker.is_fainted() || defender.is_fainted() {
            return;
        }

        if let Some(Status::Paralysis) = &attacker.status {
            let roll: f32 = rand::random();
            if roll < 0.25 {
                println!("> {} is paralyzed and can't move!", attacker.name);
                return;
            }
        }
        attacker.moves[move_index].current_pp -= 1;

        // clone the move so we don't hold a borrow on attacker while we also need to mutate defender
        let used_move = attacker.moves[move_index].clone();
        println!("\n> {} used {}! ({}/{} PP)", attacker.name, used_move.name, used_move.current_pp, used_move.max_pp);

        let accuracy_roll: f32 = rand::random::<f32>() * 100.0;
        if accuracy_roll >= used_move.accuracy as f32 {
            println!("> {}'s {} missed!", attacker.name, used_move.name);
            return;
        }
        
        for effect in &used_move.effects { // loop over effects
            match effect {
                MoveEffect::Damage => {
                    let m1 = get_type_multiplier(&used_move.move_type, &defender.primary_type);
                    let m2 = match &defender.secondary_type {
                        Some(t) => get_type_multiplier(&used_move.move_type, t),
                        None    => 1.0, // single-type pokemon: second multiplier is neutral
                    };
                    let type_multiplier = m1 * m2;

                    let stab_multiplier = if used_move.move_type == attacker.primary_type
                        || attacker.secondary_type.as_ref().map_or(false, |t| t == &used_move.move_type)
                    {
                        1.5
                    } else {
                        1.0
                    };

                    let attack_multiplier  = Self::stage_to_multiplier(attacker.stat_stages.attack);
                    let defense_multiplier = Self::stage_to_multiplier(defender.stat_stages.defense);
                    let effective_attack   = attacker.attack as f32 * attack_multiplier;
                    let effective_defense  = defender.defense as f32 * defense_multiplier;
                    let level_factor  = (2 * attacker.level / 5 + 2) as f32;
                    let random_factor = 0.85 + rand::random::<f32>() * 0.15;
                    let damage = (level_factor
                        * used_move.power as f32
                        * (effective_attack / effective_defense)
                        / 50.0
                        + 2.0)
                        * stab_multiplier
                        * type_multiplier
                        * random_factor;

                    if type_multiplier == 0.0      { println!("It has no effect!");        }
                    else if type_multiplier > 1.0  { println!("It's super effective!");    }
                    else if type_multiplier < 1.0  { println!("It's not very effective..."); }

                    if stab_multiplier > 1.0 { println!("*STAB bonus applied!*"); }

                    defender.take_damage(damage as u32);
                    println!("> {} took {} damage!", defender.name, damage as u32);
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

                MoveEffect::ModifyStat { target, stat, stages } => {
                    let target_pokemon = match target {
                        StatTarget::Opponent => &mut *defender,
                        StatTarget::User     => &mut *attacker,
                    };
                    target_pokemon.stat_stages.modify(stat, *stages);
                    let direction = if *stages > 0 { "rose" } else { "fell" };
                    println!("> {}'s {:?} {}!", target_pokemon.name, stat, direction);
                }
                MoveEffect::Heal { .. } => {
                    println!("> Healing not yet implemented.");
                }
            }
        }
    }
}