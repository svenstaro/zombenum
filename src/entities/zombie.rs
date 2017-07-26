use specs::Entity;
use specs::LazyUpdate;
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


pub fn spawn_zombie(entity: Entity,
                     lazy: &LazyUpdate,
                     zombie: Option<ZombieSpec>) {
    let spec: ZombieSpec = match zombie {
        None => Default::default(),
        Some(s) => s,
    };

    lazy.insert(entity, spec.pos);
    lazy.insert(entity, spec.vel);
    lazy.insert(entity, spec.health);
    lazy.insert(entity, spec.agility);
    lazy.insert(entity, spec.intelligence);
    lazy.insert(entity, spec.nourishment);
    lazy.insert(entity, spec.hunger);
}
