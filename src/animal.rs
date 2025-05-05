use crate::animal_type::AnimalType;

#[derive(Debug)]
pub struct Animal {
    pub id: u8,
    pub animal_type: AnimalType,
    pub name: String,
}

