use serde::Deserialize;
use std::collections::HashMap;
use crate::moves::{MoveEffect, parse_category};
use crate::moves::registry::build_move_effect_registry;
use crate::pokemon::Move;

#[derive(Debug, Deserialize)]
pub struct MoveData {
    pub name: String,
    #[serde(rename = "type")]
    pub move_type: String,
    pub category: String,
    pub power: u32,
    pub accuracy: u32,
    pub pp: u32,
}

pub fn load_moves(path: &str) -> HashMap<String, Move> {
    let file_content = std::fs::read_to_string(path)
        .expect("Could not read moves.json — make sure data/moves.json exists");

    let move_data_list: Vec<MoveData> = serde_json::from_str(&file_content)
        .expect("moves.json is not valid JSON or doesn't match expected format");

    let effect_registry = build_move_effect_registry();

    let mut moves = HashMap::new();

    for move_data in move_data_list {
        let effects = effect_registry
            .get(move_data.name.as_str())
            .cloned()
            .unwrap_or_default();

        let move_type = crate::types::parse_type(&move_data.move_type);
        let category  = parse_category(&move_data.category);

        let complete_move = Move {
            name: move_data.name.clone(),
            power: move_data.power,
            move_type,
            category,
            effects,
            max_pp: move_data.pp,
            current_pp: move_data.pp,
            accuracy: move_data.accuracy,
        };

        moves.insert(move_data.name, complete_move);
    }

    moves
}