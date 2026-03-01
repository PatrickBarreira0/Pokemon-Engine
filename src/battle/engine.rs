use crate::pokemon::{Pokemon, PokemonType};
use crate::moves::{MoveEffect, StatTarget};
use crate::types::Status;

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

    pub fn get_type_multiplier(attacker_type: &PokemonType, defender_type: &PokemonType) -> f32 {
        match (attacker_type, defender_type) {
            (PokemonType::Normal, PokemonType::Rock) => 0.5,
            (PokemonType::Normal, PokemonType::Steel) => 0.5,
            (PokemonType::Normal, PokemonType::Ghost) => 0.0,

            (PokemonType::Fire, PokemonType::Grass) => 2.0,
            (PokemonType::Fire, PokemonType::Ice) => 2.0,
            (PokemonType::Fire, PokemonType::Bug) => 2.0,
            (PokemonType::Fire, PokemonType::Steel) => 2.0,
            (PokemonType::Fire, PokemonType::Fire) => 0.5,
            (PokemonType::Fire, PokemonType::Water) => 0.5,
            (PokemonType::Fire, PokemonType::Rock) => 0.5,
            (PokemonType::Fire, PokemonType::Dragon) => 0.5,

            (PokemonType::Water, PokemonType::Fire) => 2.0,
            (PokemonType::Water, PokemonType::Ground) => 2.0,
            (PokemonType::Water, PokemonType::Rock) => 2.0,
            (PokemonType::Water, PokemonType::Water) => 0.5,
            (PokemonType::Water, PokemonType::Grass) => 0.5,
            (PokemonType::Water, PokemonType::Dragon) => 0.5,

            (PokemonType::Electric, PokemonType::Water) => 2.0,
            (PokemonType::Electric, PokemonType::Flying) => 2.0,
            (PokemonType::Electric, PokemonType::Electric) => 0.5,
            (PokemonType::Electric, PokemonType::Grass) => 0.5,
            (PokemonType::Electric, PokemonType::Dragon) => 0.5,
            (PokemonType::Electric, PokemonType::Ground) => 0.0,

            (PokemonType::Grass, PokemonType::Water) => 2.0,
            (PokemonType::Grass, PokemonType::Ground) => 2.0,
            (PokemonType::Grass, PokemonType::Rock) => 2.0,
            (PokemonType::Grass, PokemonType::Fire) => 0.5,
            (PokemonType::Grass, PokemonType::Grass) => 0.5,
            (PokemonType::Grass, PokemonType::Poison) => 0.5,
            (PokemonType::Grass, PokemonType::Flying) => 0.5,
            (PokemonType::Grass, PokemonType::Bug) => 0.5,
            (PokemonType::Grass, PokemonType::Dragon) => 0.5,
            (PokemonType::Grass, PokemonType::Steel) => 0.5,

            (PokemonType::Ice, PokemonType::Grass) => 2.0,
            (PokemonType::Ice, PokemonType::Ground) => 2.0,
            (PokemonType::Ice, PokemonType::Flying) => 2.0,
            (PokemonType::Ice, PokemonType::Dragon) => 2.0,
            (PokemonType::Ice, PokemonType::Fire) => 0.5,
            (PokemonType::Ice, PokemonType::Water) => 0.5,
            (PokemonType::Ice, PokemonType::Ice) => 0.5,
            (PokemonType::Ice, PokemonType::Steel) => 0.5,

            (PokemonType::Fighting, PokemonType::Normal) => 2.0,
            (PokemonType::Fighting, PokemonType::Ice) => 2.0,
            (PokemonType::Fighting, PokemonType::Rock) => 2.0,
            (PokemonType::Fighting, PokemonType::Dark) => 2.0,
            (PokemonType::Fighting, PokemonType::Steel) => 2.0,
            (PokemonType::Fighting, PokemonType::Poison) => 0.5,
            (PokemonType::Fighting, PokemonType::Flying) => 0.5,
            (PokemonType::Fighting, PokemonType::Psychic) => 0.5,
            (PokemonType::Fighting, PokemonType::Bug) => 0.5,
            (PokemonType::Fighting, PokemonType::Fairy) => 0.5,
            (PokemonType::Fighting, PokemonType::Ghost) => 0.0,

            (PokemonType::Poison, PokemonType::Grass) => 2.0,
            (PokemonType::Poison, PokemonType::Fairy) => 2.0,
            (PokemonType::Poison, PokemonType::Poison) => 0.5,
            (PokemonType::Poison, PokemonType::Ground) => 0.5,
            (PokemonType::Poison, PokemonType::Rock) => 0.5,
            (PokemonType::Poison, PokemonType::Ghost) => 0.5,
            (PokemonType::Poison, PokemonType::Steel) => 0.0,

            (PokemonType::Ground, PokemonType::Fire) => 2.0,
            (PokemonType::Ground, PokemonType::Electric) => 2.0,
            (PokemonType::Ground, PokemonType::Poison) => 2.0,
            (PokemonType::Ground, PokemonType::Rock) => 2.0,
            (PokemonType::Ground, PokemonType::Steel) => 2.0,
            (PokemonType::Ground, PokemonType::Grass) => 0.5,
            (PokemonType::Ground, PokemonType::Bug) => 0.5,
            (PokemonType::Ground, PokemonType::Flying) => 0.0,

            (PokemonType::Flying, PokemonType::Grass) => 2.0,
            (PokemonType::Flying, PokemonType::Fighting) => 2.0,
            (PokemonType::Flying, PokemonType::Bug) => 2.0,
            (PokemonType::Flying, PokemonType::Electric) => 0.5,
            (PokemonType::Flying, PokemonType::Rock) => 0.5,
            (PokemonType::Flying, PokemonType::Steel) => 0.5,

            (PokemonType::Psychic, PokemonType::Fighting) => 2.0,
            (PokemonType::Psychic, PokemonType::Poison) => 2.0,
            (PokemonType::Psychic, PokemonType::Psychic) => 0.5,
            (PokemonType::Psychic, PokemonType::Steel) => 0.5,
            (PokemonType::Psychic, PokemonType::Dark) => 0.0,

            (PokemonType::Bug, PokemonType::Grass) => 2.0,
            (PokemonType::Bug, PokemonType::Psychic) => 2.0,
            (PokemonType::Bug, PokemonType::Dark) => 2.0,
            (PokemonType::Bug, PokemonType::Fire) => 0.5,
            (PokemonType::Bug, PokemonType::Fighting) => 0.5,
            (PokemonType::Bug, PokemonType::Poison) => 0.5,
            (PokemonType::Bug, PokemonType::Flying) => 0.5,
            (PokemonType::Bug, PokemonType::Ghost) => 0.5,
            (PokemonType::Bug, PokemonType::Steel) => 0.5,
            (PokemonType::Bug, PokemonType::Fairy) => 0.5,

            (PokemonType::Rock, PokemonType::Fire) => 2.0,
            (PokemonType::Rock, PokemonType::Ice) => 2.0,
            (PokemonType::Rock, PokemonType::Flying) => 2.0,
            (PokemonType::Rock, PokemonType::Bug) => 2.0,
            (PokemonType::Rock, PokemonType::Fighting) => 0.5,
            (PokemonType::Rock, PokemonType::Ground) => 0.5,
            (PokemonType::Rock, PokemonType::Steel) => 0.5,

            (PokemonType::Ghost, PokemonType::Psychic) => 2.0,
            (PokemonType::Ghost, PokemonType::Ghost) => 2.0,
            (PokemonType::Ghost, PokemonType::Dark) => 0.5,
            (PokemonType::Ghost, PokemonType::Normal) => 0.0,

            (PokemonType::Dragon, PokemonType::Dragon) => 2.0,
            (PokemonType::Dragon, PokemonType::Steel) => 0.5,
            (PokemonType::Dragon, PokemonType::Fairy) => 0.0,

            (PokemonType::Dark, PokemonType::Psychic) => 2.0,
            (PokemonType::Dark, PokemonType::Ghost) => 2.0,
            (PokemonType::Dark, PokemonType::Fighting) => 0.5,
            (PokemonType::Dark, PokemonType::Dark) => 0.5,
            (PokemonType::Dark, PokemonType::Fairy) => 0.5,

            (PokemonType::Steel, PokemonType::Ice) => 2.0,
            (PokemonType::Steel, PokemonType::Rock) => 2.0,
            (PokemonType::Steel, PokemonType::Fairy) => 2.0,
            (PokemonType::Steel, PokemonType::Fire) => 0.5,
            (PokemonType::Steel, PokemonType::Water) => 0.5,
            (PokemonType::Steel, PokemonType::Electric) => 0.5,
            (PokemonType::Steel, PokemonType::Steel) => 0.5,

            (PokemonType::Fairy, PokemonType::Fighting) => 2.0,
            (PokemonType::Fairy, PokemonType::Dragon) => 2.0,
            (PokemonType::Fairy, PokemonType::Dark) => 2.0,
            (PokemonType::Fairy, PokemonType::Fire) => 0.5,
            (PokemonType::Fairy, PokemonType::Poison) => 0.5,
            (PokemonType::Fairy, PokemonType::Steel) => 0.5,

            _ => 1.0,
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
        
        for effect in &used_move.effects { //loop over effects
            match effect {
                MoveEffect::Damage => {
                    let type_multiplier = Self::get_type_multiplier(&used_move.move_type, &defender.primary_type);
                    let stab_multiplier = if used_move.move_type == attacker.primary_type {
                        1.5
                    } else {
                        1.0
                    };
                    let attack_multiplier  = Self::stage_to_multiplier(attacker.stat_stages.attack);
                    let defense_multiplier = Self::stage_to_multiplier(defender.stat_stages.defense);
                    let effective_attack   = attacker.attack as f32 * attack_multiplier;
                    let effective_defense  = defender.defense as f32 * defense_multiplier;

                    let damage = (used_move.power as f32
                        * (effective_attack / effective_defense)
                        * type_multiplier
                        * stab_multiplier) as u32;

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