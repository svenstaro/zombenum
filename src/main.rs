extern crate specs;
extern crate ggez;

mod components;
mod systems;
mod survivor;
mod zombie;

use specs::{World, RunNow};

use components::common::{Position, Velocity, Label};
use systems::{Movement, Printer};


fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Label>();

    survivor::add_survivor(&mut world, None);
    zombie::add_zombie(&mut world, None);

    let mut movement = Movement;
    movement.run_now(&world.res);

    let mut printer = Printer;
    printer.run_now(&world.res);
}
