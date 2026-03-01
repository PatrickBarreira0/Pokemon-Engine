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
        "Ice"      => crate::pokemon::PokemonType::Ice,
        "Fighting" => crate::pokemon::PokemonType::Fighting,
        "Poison"   => crate::pokemon::PokemonType::Poison,
        "Ground"   => crate::pokemon::PokemonType::Ground,
        "Flying"   => crate::pokemon::PokemonType::Flying,
        "Psychic"  => crate::pokemon::PokemonType::Psychic,
        "Bug"      => crate::pokemon::PokemonType::Bug,
        "Rock"     => crate::pokemon::PokemonType::Rock,
        "Ghost"    => crate::pokemon::PokemonType::Ghost,
        "Dragon"   => crate::pokemon::PokemonType::Dragon,
        "Dark"     => crate::pokemon::PokemonType::Dark,
        "Steel"    => crate::pokemon::PokemonType::Steel,
        "Fairy"    => crate::pokemon::PokemonType::Fairy,
        _          => panic!("Unknown type: {}", type_str),
    }
}