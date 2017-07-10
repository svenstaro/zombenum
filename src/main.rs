extern crate specs;
extern crate ggez;

mod components;
mod systems;

use specs::{World, RunNow};

use components::{Position, Velocity};
use systems::Movement;


fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    world
        .create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .with(Velocity {x: 0.5, y: 0.5})
        .build();

    let mut hello_world = Movement;
    hello_world.run_now(&world.res);
}
