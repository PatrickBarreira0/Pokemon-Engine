#[derive(Debug, Clone)]
pub enum Status {
    Burn,
    Paralysis,
    Poison,
    Freeze,
    Sleep,
}

pub fn parse_type(type_str: &str) -> crate::pokemon::PokemonType {
    match type_str {
        "Normal"   => crate::pokemon::PokemonType::Normal,
        "Fire"     => crate::pokemon::PokemonType::Fire,
        "Water"    => crate::pokemon::PokemonType::Water,
        "Grass"    => crate::pokemon::PokemonType::Grass,
        "Electric" => crate::pokemon::PokemonType::Electric,
        _          => panic!("Unknown type: {}", type_str),
    }
}