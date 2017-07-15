//! Components that living entities can be assembled from.
//! This includes 'the *living* dead', i.e. zombies.

use specs::{Component, BTreeStorage};
use components::common::Velocity;


/// When Health is 0, something is dead.
/// Otherwise, it is alive.
#[derive(Debug)]
pub struct Health {
    pub amount: u8,
}

impl Component for Health {
    type Storage = BTreeStorage<Self>;
}


/// Various physical attributes
/// TODO: is Agility a good name?
#[derive(Debug)]
pub struct Agility {
    pub max_speed: Velocity,
    pub max_strength: u8,
}

impl Component for Agility {
    type Storage = BTreeStorage<Self>;
}


/// How well a living thing is supplied with liquids, i.e. water,
/// solids, i.e. food, and the resulting energy it possesses.
#[derive(Debug)]
pub struct Nourishment {
    pub solids: u8,
    pub liquids: u8,
    pub energy: u8,
}

impl Component for Nourishment {
    type Storage = BTreeStorage<Self>;
}


/// Hunger influences behavior. Should be dependent on Nourishment levels.
#[derive(Debug)]
pub struct Hunger {
    pub level: u8,
}

impl Component for Hunger {
    type Storage = BTreeStorage<Self>;
}


/// Like hunger, but for liquids. Zombies don't feel Thist (?).
#[derive(Debug)]
pub struct Thirst {
    pub level: u8,
}

impl Component for Thirst {
    type Storage = BTreeStorage<Self>;
}
