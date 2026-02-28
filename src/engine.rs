use crate::pokemon::Pokemon;

pub struct BattleEngine {
    pub player: Pokemon,
    pub opponent: Pokemon,
}

impl BattleEngine {
    pub fn new(player: Pokemon, opponent: Pokemon) -> Self {
        Self { player, opponent }
    }

    pub fn get_type_multiplier(attacker_type: &PokemonType, defender_type: &PokemonType) -> f32 {
        match (attacker_type, defender_type) {
            (PokemonType::Fire, PokemonType::Grass) => 2.0,
            (PokemonType::Grass, PokemonType::Fire) => 0.5,

            (PokemonType::Water, PokemonType::Fire) => 2.0,
            (PokemonType::Fire, PokemonType::Water) => 0.5,
            _=> 1.0, // for each case not listed, the multiplier is 1.0
        }
    }

    pub fn execute_move(attacker: &mut Pokemon, defender: &mut Pokemon, move_index: usize) {
        if attacker.is_fainted() || defender.is_fainted() {
            return;
        }

        let used_move = &attacker.moves[move_index];

        let multiplier = Self::get_type_multiplier(&used_move.move_type, &defender.primary_type);
       
        let damage = (used_move.power as f32 * multiplier) as u32;

        println!("\n> {} used {}!", attacker.name, used_move.name);

        if multiplier > 1.0 { println!("It's super effective!"); }
        else if multiplier < 1.0 { println!("It's not very effective..."); }

        defender.take_damage(damage);
        println!("> {} took {} damage!", defender.name, damage);

    }
}