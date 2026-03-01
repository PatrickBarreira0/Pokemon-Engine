// table of contents inside moves folder and shared types
pub mod registry;
pub mod loader;

use crate::types::Status;


#[derive(Debug, Clone)]
pub enum MoveEffect {
    Damage,

    ApplyStatus {
        status: Status,
        chance: f32,
    },

    ModifyStat {
        target: StatTarget,
        stat: Stat,
        stages: i32,
    },

    Heal {
        percent: f32,
    },
}


#[derive(Debug, Clone)]
pub enum Stat {
    Attack,
    Defense,
    Speed,
    SpAttack,
    SpDefense,
}

#[derive(Debug, Clone)]
pub enum StatTarget {
    User,
    Opponent,
}