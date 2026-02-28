use std::collections::HashMap;
use super::{MoveEffect, Status, Stat, StatTarget};

pub fn build_move_effect_registry() -> HashMap<&'static str, Vec<MoveEffect>> { //static str = key or the move name, Vec<MoveEffect> = the effects of the move and the value of the key
    let mut map = HashMap::new();

    map.insert("Tackle",       vec![MoveEffect::Damage]);
    map.insert("Vine Whip",    vec![MoveEffect::Damage]);

    map.insert("Thunder Wave", vec![
        MoveEffect::ApplyStatus { status: Status::Paralysis, chance: 1.0 }
    ]);
    map.insert("Ember", vec![
        MoveEffect::Damage,
        MoveEffect::ApplyStatus { status: Status::Burn, chance: 0.1 },
    ]);
    map.insert("Growl", vec![
        MoveEffect::ModifyStat { target: StatTarget::Opponent, stat: Stat::Attack, stages: -1 }
    ]);

    map
}