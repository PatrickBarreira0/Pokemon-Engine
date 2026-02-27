use crate::pokemon::Pokemon;

pub struct BattleEngine {
    pub player: Pokemon,
    pub opponent: Pokemon,
}

impl BattleEngine {
    pub fn new(player: Pokemon, opponent: Pokemon) -> Self {
        Self { player, opponent }
    }

    pub fn execute_move(attacker: &mut Pokemon, defender: &mut Pokemon, move_index: usize) {
        if attacker.is_fainted() || defender.is_fainted() {
            return;
        }

        let used_move = &attacker.moves[move_index];
        println!("\n> {} used {}!", attacker.name, used_move.name);

        let damage = used_move.power;
        defender.take_damage(damage);

        println!(
            "> {} took {} damage! (HP: {}/{})",
            defender.name, damage, defender.current_hp, defender.max_hp
        );
    }
}