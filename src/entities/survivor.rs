use specs::World;
use specs::Entity;
use specs::LazyUpdate;
use components::common::*;
use components::living::*;


#[derive(Default)]
pub struct SurvivorSpec {
    name: Name,
    pos: Position,
    vel: Velocity,
    health: Health,
    agility: Agility,
    intelligence: Intelligence,
    nourishment: Nourishment,
    hunger: Hunger,
    thirst: Thirst,
}


pub fn spawn_survivor(entity: Entity,
                     lazy: &LazyUpdate,
                     survivor: Option<SurvivorSpec>) {
    let spec: SurvivorSpec = match survivor {
        None => Default::default(),
        Some(s) => s,
    };

    lazy.insert(entity, spec.name);
    lazy.insert(entity, spec.pos);
    lazy.insert(entity, spec.vel);
    lazy.insert(entity, spec.health);
    lazy.insert(entity, spec.agility);
    lazy.insert(entity, spec.intelligence);
    lazy.insert(entity, spec.nourishment);
    lazy.insert(entity, spec.hunger);
    lazy.insert(entity, spec.thirst);
}
