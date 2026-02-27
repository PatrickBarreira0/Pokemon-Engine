// src/pokemon.rs

#[derive(Debug, Clone)]
pub enum PokemonType {
    Normal,
}

#[derive(Debug, Clone)]
pub struct Move {
    pub name: String,
    pub power: u32,
    pub move_type: PokemonType,
}

#[derive(Debug, Clone)]
pub struct Pokemon {
    pub name: String,
    pub max_hp: u32,
    pub current_hp: u32,
    pub speed: u32,
    pub moves: Vec<Move>,
}

impl Pokemon {
    pub fn new(name: &str, max_hp: u32, speed: u32, moves: Vec<Move>) -> Self {
        Self {
            name: name.to_string(),
            max_hp,
            current_hp: max_hp,
            speed,
            moves,
        }
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.current_hp = self.current_hp.saturating_sub(amount);
    }

    pub fn is_fainted(&self) -> bool {
        self.current_hp == 0
    }
}