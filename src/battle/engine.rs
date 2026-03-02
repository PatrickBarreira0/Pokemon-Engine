use crate::pokemon::Pokemon;
use crate::moves::{MoveEffect, MoveCategory, StatTarget};
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

            _  => 1.00,
        }
    }

    pub fn process_end_of_turn(pokemon: &mut Pokemon) {
        match &pokemon.status {
            Some(Status::Burn) => {
                let burn_damage = (pokemon.max_hp / 8).max(1); // .max(1) ensures burn always deals at least 1 damage
                println!("> {} is hurt by its burn!", pokemon.name);
                pokemon.take_damage(burn_damage);
            }
            Some(Status::Poison) => {
                let poison_damage = (pokemon.max_hp / 8).max(1);
                println!("> {} is hurt by poison!", pokemon.name);
                pokemon.take_damage(poison_damage);
            }
            _ => {}
        }
    }

    pub fn execute_move(attacker: &mut Pokemon, defender: &mut Pokemon, move_index: usize) {
        if attacker.is_fainted() || defender.is_fainted() {
            return;
        }
        
        if matches!(&attacker.status, Some(Status::Sleep)) {
            match attacker.sleep_turns {
                Some(0) => {
                    attacker.status = None;
                    attacker.sleep_turns = None;
                    println!("> {} woke up!", attacker.name);
                    // falls through and moves normally this turn
                }
                Some(n) => {
                    attacker.sleep_turns = Some(n - 1);
                    println!("> {} is fast asleep!", attacker.name);
                    return;
                }
                None => {}
            }
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
        for effect in &used_move.effects {
            match effect {
                MoveEffect::Damage => {
                    let m1 = get_type_multiplier(&used_move.move_type, &defender.primary_type);
                    let m2 = match &defender.secondary_type {
                        Some(t) => get_type_multiplier(&used_move.move_type, t),
                        None    => 1.0,
                    };
                    let type_multiplier = m1 * m2;

                    let stab_multiplier = if used_move.move_type == attacker.primary_type
                        || attacker.secondary_type.as_ref().map_or(false, |t| t == &used_move.move_type)
                    {
                        1.5
                    } else {
                        1.0
                    };

                    let is_crit = rand::random::<f32>() < (1.0 / 24.0);
                    let critical = if is_crit { 2.0 } else { 1.0 };

                    let (effective_attack, effective_defense) = match used_move.category {
                        MoveCategory::Physical => {
                            // on a crit, attacker's negative attack stages are ignored (clamped to 0)
                            // and defender's positive defense stages are ignored (clamped to 0)
                            let atk_stage = if is_crit { attacker.stat_stages.attack.max(0)  } else { attacker.stat_stages.attack };
                            let def_stage = if is_crit { defender.stat_stages.defense.min(0) } else { defender.stat_stages.defense };
                            let atk = attacker.attack  as f32 * Self::stage_to_multiplier(atk_stage);
                            let def = defender.defense as f32 * Self::stage_to_multiplier(def_stage);
                            (atk, def)
                        }
                        MoveCategory::Special => {
                            let atk_stage = if is_crit { attacker.stat_stages.sp_attack.max(0)  } else { attacker.stat_stages.sp_attack };
                            let def_stage = if is_crit { defender.stat_stages.sp_defense.min(0) } else { defender.stat_stages.sp_defense };
                            let atk = attacker.sp_attack  as f32 * Self::stage_to_multiplier(atk_stage);
                            let def = defender.sp_defense as f32 * Self::stage_to_multiplier(def_stage);
                            (atk, def)
                        }
                        MoveCategory::Status => unreachable!("Status moves should not have a Damage effect"),
                    };

                    let burn = if matches!(&attacker.status, Some(Status::Burn))
                        && used_move.category == MoveCategory::Physical
                    {
                        0.5
                    } else {
                        1.0
                    };

                    // step1: floor(2 * level / 5 + 2)
                    let step1 = 2 * attacker.level / 5 + 2;
                    // step2: floor(step1 * power * atk / def)
                    let step2 = (step1 as f32 * used_move.power as f32 * effective_attack / effective_defense) as u32;
                    // step3: floor(step2 / 50) + 2
                    let step3 = step2 / 50 + 2;
                    // final: floor(step3 * critical * random * stab * type * burn)
                    // weather, targets, other modifiers stubbed as 1.0 for now
                    let random_factor = 0.85 + rand::random::<f32>() * 0.15;
                    let damage = (step3 as f32 * critical * random_factor * stab_multiplier * type_multiplier * burn) as u32;

                    if type_multiplier == 0.0     { println!("It has no effect!");            }
                    else if type_multiplier > 1.0 { println!("It's super effective!");        }
                    else if type_multiplier < 1.0 { println!("It's not very effective...");   }
                    if is_crit                    { println!("A critical hit!");               }
                    if stab_multiplier > 1.0      { println!("*STAB bonus applied!*");        }

                    defender.take_damage(damage);
                    println!("> {} took {} damage!", defender.name, damage);
                }

                MoveEffect::ApplyStatus { status, chance } => {
                    let roll: f32 = rand::random();
                    if roll < *chance {

                        if defender.status.is_none() {
                            println!("> {} was afflicted with {:?}!", defender.name, status);
                            defender.status = Some(status.clone());

                            if matches!(status, Status::Sleep) {
                                let turns = (rand::random::<f32>() * 3.0) as u32 + 1;
                                defender.sleep_turns = Some(turns);
                            }
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