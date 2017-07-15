use specs::World;
use components::{Position, Velocity, Label};

pub struct SurvivorSpec {
    pub label: Label,
    pub pos: Position,
    pub vel: Velocity,
}

impl SurvivorSpec {
    pub fn new(pos: Position, vel: Velocity) -> SurvivorSpec {
        SurvivorSpec {
            label: Label::Survivor,
            pos: pos,
            vel: vel,
        }
    }

    pub fn default() -> SurvivorSpec {
        SurvivorSpec {
            label: Label::Survivor,
            pos: Position { x: 0.0, y: 0.0 },
            vel: Velocity { x: 0.0, y: 0.0 },
        }
    }
}

pub fn add_survivor(world: &mut World, survivor: Option<SurvivorSpec>) {
    let spec = match survivor {
        None => SurvivorSpec::default(),
        Some(s) => s,
    };

    world
        .create_entity()
        .with(spec.label)
        .with(spec.pos)
        .with(spec.vel)
        .build();
}
