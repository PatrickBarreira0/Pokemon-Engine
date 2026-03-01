use serde::Deserialize;
use crate::pokemon::{Pokemon, Nature};
use crate::moves::Stat;
use crate::moves::loader::load_moves;

#[derive(Debug, Deserialize)]
pub struct IVsData {
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub sp_attack: u32,
    pub sp_defense: u32,
    pub speed: u32,
}

#[derive(Debug, Deserialize)]
pub struct EVsData {
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub sp_attack: u32,
    pub sp_defense: u32,
    pub speed: u32,
}

#[derive(Debug, Deserialize)]
pub struct PokemonData {
    pub name: String,
    #[serde(rename = "type")]
    pub pokemon_type: String,
    pub secondary_type: Option<String>,
    pub level: u32,
    pub nature: String,
    pub ivs: IVsData,
    pub evs: EVsData,
    pub base_hp: u32,
    pub base_attack: u32,
    pub base_defense: u32,
    pub base_sp_attack: u32,
    pub base_sp_defense: u32,
    pub base_speed: u32,
    pub moves: Vec<String>,
}

// Gen 5 HP formula: floor((2*base + iv + floor(ev/4)) * level / 100) + level + 10
fn calc_hp(base: u32, iv: u32, ev: u32, level: u32) -> u32 {
    let inner = (2 * base + iv + ev / 4) * level / 100;
    inner + level + 10
}

// Gen 5 stat formula: floor((floor((2*base + iv + floor(ev/4)) * level / 100) + 5) * nature_multiplier)
fn calc_stat(base: u32, iv: u32, ev: u32, level: u32, nature: &Nature, stat: &Stat) -> u32 {
    let inner = (2 * base + iv + ev / 4) * level / 100 + 5;
    let multiplier = crate::pokemon::nature_multiplier(nature, stat);
    (inner as f32 * multiplier) as u32
}

fn parse_nature(s: &str) -> Nature {
    match s {
        "Hardy"   => Nature::Hardy,
        "Lonely"  => Nature::Lonely,
        "Brave"   => Nature::Brave,
        "Adamant" => Nature::Adamant,
        "Naughty" => Nature::Naughty,
        "Bold"    => Nature::Bold,
        "Docile"  => Nature::Docile,
        "Relaxed" => Nature::Relaxed,
        "Impish"  => Nature::Impish,
        "Lax"     => Nature::Lax,
        "Timid"   => Nature::Timid,
        "Hasty"   => Nature::Hasty,
        "Serious" => Nature::Serious,
        "Jolly"   => Nature::Jolly,
        "Naive"   => Nature::Naive,
        "Modest"  => Nature::Modest,
        "Mild"    => Nature::Mild,
        "Quiet"   => Nature::Quiet,
        "Bashful" => Nature::Bashful,
        "Rash"    => Nature::Rash,
        "Calm"    => Nature::Calm,
        "Gentle"  => Nature::Gentle,
        "Sassy"   => Nature::Sassy,
        "Careful" => Nature::Careful,
        "Quirky"  => Nature::Quirky,
        other     => panic!("Unknown nature: {}", other),
    }
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

        let primary_type   = crate::types::parse_type(&data.pokemon_type);
        let secondary_type = data.secondary_type.as_deref().map(crate::types::parse_type);

        let nature         = parse_nature(&data.nature);
        let max_hp     = calc_hp  (data.base_hp,         data.ivs.hp,         data.evs.hp,         data.level);
        let attack     = calc_stat(data.base_attack,     data.ivs.attack,     data.evs.attack,     data.level, &nature, &Stat::Attack);
        let defense    = calc_stat(data.base_defense,    data.ivs.defense,    data.evs.defense,    data.level, &nature, &Stat::Defense);
        let sp_attack  = calc_stat(data.base_sp_attack,  data.ivs.sp_attack,  data.evs.sp_attack,  data.level, &nature, &Stat::SpAttack);
        let sp_defense = calc_stat(data.base_sp_defense, data.ivs.sp_defense, data.evs.sp_defense, data.level, &nature, &Stat::SpDefense);
        let speed      = calc_stat(data.base_speed,      data.ivs.speed,      data.evs.speed,      data.level, &nature, &Stat::Speed);

        let pokemon = Pokemon::new(
            &data.name,
            primary_type,
            secondary_type,
            data.level,
            max_hp,
            attack,
            defense,
            sp_attack,
            sp_defense,
            speed,
            moves,
        );

        pokemon_list.push(pokemon);
    }

    pokemon_list
}