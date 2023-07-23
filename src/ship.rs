use serde::Deserialize;

pub type Ships = Vec<Ship>;

#[derive(Debug, Deserialize)]
pub struct Ship {
    
    pub symbol: String,
    // pub x: i32,
    // pub y: i32,
}

#[derive(Debug, Deserialize)]
pub struct Crew {
    pub current: i32,
    pub capacity: i32,
    pub required: i32,
    pub moral : i32,
    pub wages  : i32,
}

#[derive(Debug, Deserialize)]
pub struct Fuel {
    pub current: i32,
    pub capacity: i32,
}

#[derive(Debug, Deserialize)]
pub struct Cargo {
    pub capacity: i32,
    pub units: i32,
}