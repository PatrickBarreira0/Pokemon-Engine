// src/pokemon/mod.rs
use crate::moves::MoveEffect;
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
}

#[derive(Debug, Clone)]
pub struct Move {
    pub name: String,
    pub power: u32,
    pub move_type: PokemonType,
    pub effects: Vec<MoveEffect>,
    pub max_pp: u32,
    pub current_pp: u32,
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
            Stat::Attack     => self.attack     = (self.attack     + amount).clamp(-6, 6),
            Stat::Defense    => self.defense    = (self.defense    + amount).clamp(-6, 6),
            Stat::Speed      => self.speed      = (self.speed      + amount).clamp(-6, 6),
            Stat::SpAttack   => self.sp_attack  = (self.sp_attack  + amount).clamp(-6, 6),
            Stat::SpDefense  => self.sp_defense = (self.sp_defense + amount).clamp(-6, 6),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pokemon {
    pub name: String,
    pub primary_type: PokemonType,
    pub max_hp: u32,
    pub current_hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub speed: u32,
    pub moves: Vec<Move>,
    pub status: Option<Status>,
    pub stat_stages: StatStages,
}

impl Pokemon {
    pub fn new(name: &str, p_type: PokemonType, max_hp: u32, attack: u32, defense: u32, speed: u32, moves: Vec<Move>) -> Self {
        Self {
            name: name.to_string(),
            primary_type: p_type,
            max_hp,
            current_hp: max_hp,
            attack,
            defense,
            speed,
            moves,
            status: None,
            stat_stages: StatStages::new(),
        }
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.current_hp = self.current_hp.saturating_sub(amount);
    }

    pub fn is_fainted(&self) -> bool {
        self.current_hp == 0
    }
}