use specs::World;
use components::{Position, Velocity, Label};

pub struct ZombieSpec {
    pub label: Label,
    pub pos: Position,
    pub vel: Velocity,
}

impl ZombieSpec {
    pub fn new(pos: Position, vel: Velocity) -> ZombieSpec {
        ZombieSpec {
            label: Label::Zombie,
            pos: pos,
            vel: vel,
        }
    }

    pub fn default() -> ZombieSpec {
        ZombieSpec {
            label: Label::Zombie,
            pos: Position { x: 0.0, y: 0.0 },
            vel: Velocity { x: 0.0, y: 0.0 },
        }
    }
}

pub fn add_zombie(world: &mut World, survivor: Option<ZombieSpec>) {
    let spec = match survivor {
        None => ZombieSpec::default(),
        Some(s) => s,
    };

    world
        .create_entity()
        .with(spec.label)
        .with(spec.pos)
        .with(spec.vel)
        .build();
}
