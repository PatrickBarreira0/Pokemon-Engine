// src/pokemon.rs
use crate::moves::MoveEffect;
use crate::types::Status;

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
            status: None
        }
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.current_hp = self.current_hp.saturating_sub(amount);
    }

    pub fn is_fainted(&self) -> bool {
        self.current_hp == 0
    }
}