use specs::World;
use components::common::*;
use components::living::*;


#[derive(Default)]
pub struct SurvivorSpec {
    pos: Position,
    vel: Velocity,
    health: Health,
    agility: Agility,
    intelligence: Intelligence,
    nourishment: Nourishment,
    hunger: Hunger,
    thirst: Thirst,
}

impl SurvivorSpec {
    pub fn new(pos: Position,
               vel: Velocity,
               health: Health,
               agility: Agility,
               intelligence: Intelligence,
               nourishment: Nourishment,
               hunger: Hunger,
               thirst: Thirst
               ) -> SurvivorSpec {
        SurvivorSpec {
            pos: pos,
            vel: vel,
            health: health,
            agility: agility,
            intelligence: intelligence,
            nourishment: nourishment,
            hunger: hunger,
            thirst: thirst,
        }
    }
}

pub fn add_survivor(world: &mut World, survivor: Option<SurvivorSpec>) {
    let spec: SurvivorSpec = match survivor {
        None => Default::default(),
        Some(s) => s,
    };

    world
        .create_entity()
        .with(spec.pos)
        .with(spec.vel)
        .with(spec.intelligence)
        .build();
}
