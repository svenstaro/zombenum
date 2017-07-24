use specs::World;
use components::common::*;
use components::living::*;

pub struct ZombieSpec {
    pos: Position,
    vel: Velocity,
    health: Health,
    agility: Agility,
    intelligence: Intelligence,
    nourishment: Nourishment,
    hunger: Hunger,
}

impl ZombieSpec {
    pub fn new(pos: Position,
               vel: Velocity,
               health: Health,
               agility: Agility,
               intelligence: Intelligence,
               nourishment: Nourishment,
               hunger: Hunger
               ) -> ZombieSpec {
        ZombieSpec {
            pos: pos,
            vel: vel,
            health: health,
            agility: agility,
            intelligence: intelligence,
            nourishment: nourishment,
            hunger: hunger,
        }
    }
}

impl Default for ZombieSpec {
    fn default() -> ZombieSpec {
        ZombieSpec {
            pos: Default::default(),
            vel: Default::default(),
            health: Health { amount: 120 },
            agility: Agility { max_speed: 8.0, max_strength: 120 },
            intelligence: Intelligence { iq: 50 },
            nourishment: Nourishment { solids: 80, liquids: 80, energy: 80 },
            hunger: Hunger { level: 20 },
        }
    }
}


pub fn add_zombie(world: &mut World, survivor: Option<ZombieSpec>) {
    let spec: ZombieSpec = match survivor {
        None => Default::default(),
        Some(s) => s,
    };

    world
        .create_entity()
        .with(spec.pos)
        .with(spec.vel)
        .with(spec.intelligence)
        .build();

    trace!("zombie entity created!");
}
