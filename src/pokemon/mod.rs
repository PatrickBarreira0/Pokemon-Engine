use crate::moves::{MoveEffect, MoveCategory};
use crate::types::Status;
use crate::moves::Stat;
pub mod loader;

#[derive(Debug, Clone, PartialEq)]
pub enum PokemonType {
    Normal,
    Fire,
    Water,
    Grass,
    Electric,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
}

#[derive(Debug, Clone)]
pub enum Nature {
    Hardy,   // neutral
    Lonely,  // +Atk, -Def
    Brave,   // +Atk, -Spe
    Adamant, // +Atk, -SpA
    Naughty, // +Atk, -SpD
    Bold,    // +Def, -Atk
    Docile,  // neutral
    Relaxed, // +Def, -Spe
    Impish,  // +Def, -SpA
    Lax,     // +Def, -SpD
    Timid,   // +Spe, -Atk
    Hasty,   // +Spe, -Def
    Serious, // neutral
    Jolly,   // +Spe, -SpA
    Naive,   // +Spe, -SpD
    Modest,  // +SpA, -Atk
    Mild,    // +SpA, -Def
    Quiet,   // +SpA, -Spe
    Bashful, // neutral
    Rash,    // +SpA, -SpD
    Calm,    // +SpD, -Atk
    Gentle,  // +SpD, -Def
    Sassy,   // +SpD, -Spe
    Careful, // +SpD, -SpA
    Quirky,  // neutral
}

pub fn nature_multiplier(nature: &Nature, stat: &Stat) -> f32 {
    let (boosted, lowered) = match nature {
        Nature::Hardy   => (Stat::Attack,    Stat::Attack),    // neutral
        Nature::Lonely  => (Stat::Attack,    Stat::Defense),
        Nature::Brave   => (Stat::Attack,    Stat::Speed),
        Nature::Adamant => (Stat::Attack,    Stat::SpAttack),
        Nature::Naughty => (Stat::Attack,    Stat::SpDefense),
        Nature::Bold    => (Stat::Defense,   Stat::Attack),
        Nature::Docile  => (Stat::Defense,   Stat::Defense),   // neutral
        Nature::Relaxed => (Stat::Defense,   Stat::Speed),
        Nature::Impish  => (Stat::Defense,   Stat::SpAttack),
        Nature::Lax     => (Stat::Defense,   Stat::SpDefense),
        Nature::Timid   => (Stat::Speed,     Stat::Attack),
        Nature::Hasty   => (Stat::Speed,     Stat::Defense),
        Nature::Serious => (Stat::Speed,     Stat::Speed),     // neutral
        Nature::Jolly   => (Stat::Speed,     Stat::SpAttack),
        Nature::Naive   => (Stat::Speed,     Stat::SpDefense),
        Nature::Modest  => (Stat::SpAttack,  Stat::Attack),
        Nature::Mild    => (Stat::SpAttack,  Stat::Defense),
        Nature::Quiet   => (Stat::SpAttack,  Stat::Speed),
        Nature::Bashful => (Stat::SpAttack,  Stat::SpAttack),  // neutral
        Nature::Rash    => (Stat::SpAttack,  Stat::SpDefense),
        Nature::Calm    => (Stat::SpDefense, Stat::Attack),
        Nature::Gentle  => (Stat::SpDefense, Stat::Defense),
        Nature::Sassy   => (Stat::SpDefense, Stat::Speed),
        Nature::Careful => (Stat::SpDefense, Stat::SpAttack),
        Nature::Quirky  => (Stat::SpDefense, Stat::SpDefense), // neutral
    };

    // since Stat doesn't derive PartialEq, we match on pairs of variants directly
    let stat_matches = |a: &Stat, b: &Stat| matches!(
        (a, b),
        (Stat::Attack,    Stat::Attack)    |
        (Stat::Defense,   Stat::Defense)   |
        (Stat::Speed,     Stat::Speed)     |
        (Stat::SpAttack,  Stat::SpAttack)  |
        (Stat::SpDefense, Stat::SpDefense)
    );

    if stat_matches(stat, &boosted) && !stat_matches(stat, &lowered) {
        1.1
    } else if stat_matches(stat, &lowered) && !stat_matches(stat, &boosted) {
        0.9
    } else {
        1.0 // neutral natures: boosted == lowered so both conditions above are false
    }
}

#[derive(Debug, Clone)]
pub struct Move {
    pub name: String,
    pub power: u32,
    pub move_type: PokemonType,
    pub category: MoveCategory,
    pub effects: Vec<MoveEffect>,
    pub max_pp: u32,
    pub current_pp: u32,
    pub accuracy: u32,
}

#[derive(Debug, Clone)]
pub struct StatStages {
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub sp_attack: i32,
    pub sp_defense: i32,
}

impl StatStages {
    pub fn new() -> Self {
        Self {
            attack: 0,
            defense: 0,
            speed: 0,
            sp_attack: 0,
            sp_defense: 0,
        }
    }

    pub fn modify(&mut self, stat: &Stat, amount: i32) {
        match stat {
            Stat::Attack    => self.attack     = (self.attack     + amount).clamp(-6, 6),
            Stat::Defense   => self.defense    = (self.defense    + amount).clamp(-6, 6),
            Stat::Speed     => self.speed      = (self.speed      + amount).clamp(-6, 6),
            Stat::SpAttack  => self.sp_attack  = (self.sp_attack  + amount).clamp(-6, 6),
            Stat::SpDefense => self.sp_defense = (self.sp_defense + amount).clamp(-6, 6),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pokemon {
    pub name: String,
    pub primary_type: PokemonType,
    pub secondary_type: Option<PokemonType>,
    pub max_hp: u32,
    pub current_hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub sp_attack: u32,
    pub sp_defense: u32,
    pub speed: u32,
    pub moves: Vec<Move>,
    pub status: Option<Status>,
    pub sleep_turns: Option<u32>,
    pub stat_stages: StatStages,
    pub level: u32,
}

impl Pokemon {
    pub fn new(
        name: &str,
        primary_type: PokemonType,
        secondary_type: Option<PokemonType>,
        level: u32,
        max_hp: u32,
        attack: u32,
        defense: u32,
        sp_attack: u32,
        sp_defense: u32,
        speed: u32,
        moves: Vec<Move>,
    ) -> Self {
        Self {
            name: name.to_string(),
            primary_type,
            secondary_type,
            max_hp,
            current_hp: max_hp,
            attack,
            defense,
            sp_attack,
            sp_defense,
            speed,
            moves,
            status: None,
            sleep_turns: None,
            stat_stages: StatStages::new(),
            level,
        }
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.current_hp = self.current_hp.saturating_sub(amount);
    }

    pub fn is_fainted(&self) -> bool {
        self.current_hp == 0
    }
}