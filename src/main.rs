// modules
mod pokemon;
mod battle;
mod moves;
mod types;

// imports
use pokemon::loader::load_pokemon;

fn main() {
    let all_pokemon = load_pokemon("data/pokemon.json", "data/moves.json");

    let bulbasaur = all_pokemon.iter()
        .find(|p| p.name == "Bulbasaur")
        .expect("Bulbasaur not found in pokemon.json")
        .clone();

    let charmander = all_pokemon.iter()
        .find(|p| p.name == "Charmander")
        .expect("Charmander not found in pokemon.json")
        .clone();
    
    battle::run_battle(bulbasaur, charmander);
}