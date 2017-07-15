use specs::{Component, BTreeStorage};


/// Simply a position in our 2.5D World. Anything and everything can have one.
///
/// There's a z-axis because houses and the like can have
/// two stories, and we might want to simulate terrain more accurately.
///
/// x and y are definitely the more important coordinates.
#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Component for Position {
    type Storage = BTreeStorage<Self>;
}


/// Almost anything can have a velocity.
///
/// It's simply a vector that is added to the Position of an entity.
/// So, something which has a Velocity must also have a Position.
#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Component for Velocity {
    type Storage = BTreeStorage<Self>;
}


/// Something to label entities with.
#[derive(Debug)]
pub enum Label {
    Zombie,
    Survivor,
    Misc,
}

impl Component for Label {
    type Storage = BTreeStorage<Self>;
}
