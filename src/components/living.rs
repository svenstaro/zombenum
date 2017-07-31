//! Components that living entities can be assembled from.
//! This includes 'the *living* dead', i.e. zombies.

use std::fmt;
use specs::{Component, BTreeStorage};
use components::common::Velocity;
use components::usable::Damage;
use util::NameGenerator;


pub struct Name {
    pub firstname: String,
    pub lastname: String,
}

impl Component for Name {
    type Storage = BTreeStorage<Self>;
}

impl Default for Name {
    fn default() -> Name {
        NameGenerator::gen_name()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.firstname, self.lastname)
    }
}


/// When Health is 0, something is dead.
/// Otherwise, it is alive.
#[derive(Debug)]
pub struct Health {
    pub amount: u8,
}

impl Health {
    pub fn apply_damage(&mut self, dmg: Damage) {
        if dmg.damage > self.amount {
            self.amount = 0;
        } else {
            self.amount -= dmg.damage;
        }
    }
}

impl Component for Health {
    type Storage = BTreeStorage<Self>;
}

impl Default for Health {
    fn default() -> Health {
        Health {
            amount: 100,
        }
    }
}


/// Various physical attributes
/// TODO: is Agility a good name?
#[derive(Debug)]
pub struct Agility {
    pub max_speed: f32,
    pub max_strength: u8,
}

impl Component for Agility {
    type Storage = BTreeStorage<Self>;
}

impl Default for Agility {
    fn default() -> Agility {
        Agility {
            max_speed: 10.0,
            max_strength: 100,
        }
    }
}


/// Intelligence as described by an IQ.
/// The average IQ is 100. That of Zombies should be much lower.
#[derive(Debug)]
pub struct Intelligence {
    pub iq: u8,
}

impl Component for Intelligence {
    type Storage = BTreeStorage<Self>;
}

impl Default for Intelligence {
    fn default() -> Intelligence {
        Intelligence {
            iq: 100,
        }
    }
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

impl Default for Nourishment {
    fn default() -> Nourishment {
        Nourishment {
            solids: 50,
            liquids: 50,
            energy: 50,
        }
    }
}


/// Hunger influences behavior. Should be dependent on Nourishment levels.
#[derive(Debug)]
pub struct Hunger {
    pub level: u8,
}

impl Component for Hunger {
    type Storage = BTreeStorage<Self>;
}

impl Default for Hunger {
    fn default() -> Hunger {
        Hunger {
            level: 10,
        }
    }
}


/// Like hunger, but for liquids. Zombies don't feel Thist (?).
#[derive(Debug)]
pub struct Thirst {
    pub level: u8,
}

impl Component for Thirst {
    type Storage = BTreeStorage<Self>;
}

impl Default for Thirst {
    fn default() -> Thirst {
        Thirst {
            level: 10,
        }
    }
}
