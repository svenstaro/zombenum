extern crate specs;
extern crate ggez;
#[macro_use]
extern crate serde_json;

mod components;
mod systems;
mod survivor;
mod zombie;

use specs::{World, RunNow};

use components::common::{Position, Velocity};
use components::living::*;
use systems::{Movement, Printer};
use systems::broadcast::TcpBroadcast;


fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Intelligence>();

    survivor::add_survivor(&mut world, None);
    zombie::add_zombie(&mut world, None);

    let mut movement = Movement;
    movement.run_now(&world.res);

    let mut printer = Printer;
    printer.run_now(&world.res);

    let mut tcp_broadcast = TcpBroadcast;
    tcp_broadcast.run_now(&world.res);
}
