#[derive(Debug, Clone)]
pub enum PokemonType {
    Fire,
    Water,
    Grass,
    Electric,
    Flying,
    Normal,
    Bug,
    Poison,
    Ground,
    Rock,
    Fighting,
    Psychic,
    Ghost,
    Ice,
    Dragon,
    Dark,
    Steel,
}

#[derive(Debug, Clone)]
pub enum PokemonNature {
    Hardy,
    Lonely,
    Brave,
    Adamant,
    Naughty,
    Bold,
    Docile,
    Relaxed,
    Impish,
    Lax,
    Timid,
    Hasty,
    Serious,
    Jolly,
    Naive,
    Modest,
    Mild,
    Quiet,
    Bashful,
    Rash,
    Calm,
    Gentle,
    Sassy,
    Careful,
    Quirky,
}

#[derive(Debug, Clone)]
pub enum PokemonEggGroup {
    Monster,
    WaterA,
    WaterB,
    WaterC,
    Bug,
    Flying,
    Field,
    Fairy,
    Plant,
    Humanoid,
    Mineral,
    Chaos,
    Ditto,
    Dragon,
    CannotBreed,
    Genderless,
}

#[derive(Debug)]
pub enum PokemonGender {
    Female,
    Male,
    Genderless,
}

#[derive(Debug, Clone)]
pub struct Pokemon {
    number: u8,
    name: String,
    types: (PokemonType, Option<PokemonType>),
    egg_groups: (PokemonEggGroup, Option<PokemonEggGroup>),
    nature: Option<PokemonNature>,
    percentage_male: u8,
}