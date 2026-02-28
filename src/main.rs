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
        vec![vine_whip, tackle.clone()],
    );
    let charmander = Pokemon::new(
        "Charmander",
        PokemonType::Fire,
        39,
        65,
        vec![ember, tackle.clone()],
    );

    let mut engine = BattleEngine::new(bulbasaur, charmander);

    println!("Battle start! {} vs {}", engine.player.name, engine.opponent.name);

    loop {
        println!("\n--- NEW TURN ---");
        println!("What will {} do?", engine.player.name);

        for (index, pokemon_move) in engine.player.moves.iter().enumerate() { // index is position in the array, pokemon_move is the move at that position
            println!("{}. {}", index + 1, pokemon_move.name); // +1 because arrays start at 0 but humans start at 1
        }
        print!("Choose: ");
        io::stdout().flush().unwrap(); 

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parsed_input = input.trim().parse::<usize>();

        if let Ok(choice) = parsed_input { // checks if the input is valid 
            if choice > 0 && choice <= engine.player.moves.len() {
                
                let player_move_index = choice - 1; // -1 to get the index of the move
                let opponent_move_index = 0; // ai always use first move for now;

                let player_is_faster = engine.player.speed >= engine.opponent.speed;

                if player_is_faster { // player goes first
                    BattleEngine::execute_move(&mut engine.player, &mut engine.opponent, player_move_index);
                    if engine.opponent.is_fainted() {
                        println!("\n*** {} fainted! You win! ***", engine.opponent.name);
                        break;
                    } // opponent retaliates

                    BattleEngine::execute_move(&mut engine.opponent, &mut engine.player, opponent_move_index);
                    if engine.player.is_fainted() {
                        println!("\n*** {} fainted! You lose! ***", engine.player.name);
                        break;
                    }
                    
                } else { // opponent goes first
                    BattleEngine::execute_move(&mut engine.opponent, &mut engine.player, opponent_move_index);
                    if engine.player.is_fainted() {
                        println!("\n*** {} fainted! You lose! ***", engine.player.name);
                        break;
                    } // player retaliates
                    BattleEngine::execute_move(&mut engine.player, &mut engine.opponent, player_move_index);
                    if engine.opponent.is_fainted() {
                        println!("\n*** {} fainted! You win! ***", engine.opponent.name);
                        break;
                    }
                }
            } else {
                println!("Invalid move number! Choose a number from the list.");
            }
        } else {
            println!("Please enter a valid number!");
        }
    }
}