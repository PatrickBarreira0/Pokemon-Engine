//modules
mod pokemon;
mod engine;

//imports
use pokemon::{Move, Pokemon, PokemonType};
use engine::BattleEngine;
use std::io::{self, Write};

fn main() {
    let tackle = Move {
        name: "Tackle".to_string(),
        power: 10,
        move_type: PokemonType::Normal,
    };
    let vine_whip = Move {
        name: "Vine Whip".to_string(),
        power: 10,
        move_type: PokemonType::Grass,
    };
    let ember = Move {
        name: "Ember".to_string(),
        power: 10,
        move_type: PokemonType::Fire,
    };
    let water_gun = Move {
        name: "Water Gun".to_string(),
        power: 10,
        move_type: PokemonType::Water,
    };

    let bulbasaur = Pokemon::new(
        "Bulbasaur",
        PokemonType::Grass,
        45,
        45,
        vec![vine_whip],
    );
    let charmander = Pokemon::new(
        "Charmander",
        PokemonType::Fire,
        39,
        65,
        vec![ember],
    );

    let mut engine = BattleEngine::new(bulbasaur, charmander);

    println!("Battle start! {} vs {}", engine.player.name, engine.opponent.name);

    loop {
        println!("\n--- NEW TURN ---");
        println!("What will {} do?", engine.player.name);
        println!("1. Tackle");
        print!("Choose: ");
        io::stdout().flush().unwrap(); 

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "1" {
            // Determine who is faster
            let player_is_faster = engine.player.speed >= engine.opponent.speed;

            if player_is_faster {
                // Player goes first
                BattleEngine::execute_move(&mut engine.player, &mut engine.opponent, 0);
                if engine.opponent.is_fainted() {
                    println!("\n*** {} fainted! You win! ***", engine.opponent.name);
                    break;
                }
                // Opponent retaliates
                BattleEngine::execute_move(&mut engine.opponent, &mut engine.player, 0);
            } else {
                // Opponent goes first (This will happen because Charmander is 65!)
                BattleEngine::execute_move(&mut engine.opponent, &mut engine.player, 0);
                if engine.player.is_fainted() {
                    println!("\n*** {} fainted! You lose! ***", engine.player.name);
                    break;
                }
                // Player retaliates
                BattleEngine::execute_move(&mut engine.player, &mut engine.opponent, 0);
            }

            // Check if player fainted after the second move
            if engine.player.is_fainted() || engine.opponent.is_fainted() {
                if engine.player.is_fainted() { println!("\n*** You lost! ***"); }
                else { println!("\n*** You won! ***"); }
                break;
            }
        }
    }
}