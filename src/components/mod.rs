use specs::{Component, BTreeStorage};


#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {
    type Storage = BTreeStorage<Self>;
}


#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {
    type Storage = BTreeStorage<Self>;
}


#[derive(Debug)]
pub enum Label {
    Zombie,
    Survivor,
    Misc,
}

impl Component for Label {
    type Storage = BTreeStorage<Self>;
}
