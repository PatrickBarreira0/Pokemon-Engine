use serde::Deserialize;
use crate::pokemon::{Pokemon, PokemonType};
use crate::moves::loader::load_moves;

#[derive(Debug, Deserialize)]
pub struct PokemonData {
    pub name: String,
    #[serde(rename = "type")]
    pub pokemon_type: String,
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub speed: u32,
    pub moves: Vec<String>,
}

pub fn load_pokemon(pokemon_path: &str, moves_path: &str) -> Vec<Pokemon> {

    let all_moves = load_moves(moves_path);

    let file_content = std::fs::read_to_string(pokemon_path)
        .expect("Could not read pokemon.json — make sure data/pokemon.json exists");

    let pokemon_data_list: Vec<PokemonData> = serde_json::from_str(&file_content)
        .expect("pokemon.json is not valid JSON or doesn't match expected format");

    let mut pokemon_list = Vec::new();

    for data in pokemon_data_list {
        let moves = data.moves.iter().map(|move_name| {
            all_moves
                .get(move_name.as_str())
                .expect(&format!("{} not found in moves.json", move_name))
                .clone()
        }).collect();

        let pokemon_type = parse_type(&data.pokemon_type);

        let pokemon = Pokemon::new(
            &data.name,
            pokemon_type,
            data.hp,
            data.attack,
            data.defense,
            data.speed,
            moves,
        );

        pokemon_list.push(pokemon);
    }

    pokemon_list
}

fn parse_type(type_str: &str) -> PokemonType {
    match type_str {
        "Normal"   => PokemonType::Normal,
        "Fire"     => PokemonType::Fire,
        "Water"    => PokemonType::Water,
        "Grass"    => PokemonType::Grass,
        "Electric" => PokemonType::Electric,
        _          => panic!("Unknown type in pokemon.json: {}", type_str),
    }
}