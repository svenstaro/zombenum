extern crate specs;
extern crate ggez;
#[macro_use]
extern crate serde_json;

mod components;
mod systems;
mod survivor;
mod zombie;

use specs::{World, RunNow, DispatcherBuilder};

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

    let mut dispatcher = DispatcherBuilder::new()
        .add(Movement, "movement", &[])
        .add(Printer, "printer", &[])
        .add(TcpBroadcast, "tcp_broadcast", &[])
        .build();

    dispatcher.dispatch(&mut world.res);
}
