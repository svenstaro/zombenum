extern crate specs;
extern crate ggez;
extern crate env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;


mod components;
mod systems;
mod resources;
mod survivor;
mod zombie;


use specs::{World, RunNow, DispatcherBuilder};

use components::common::{Position, Velocity};
use components::living::*;

use systems::{Movement, Printer, ZombieSpawner};
use systems::broadcast::TcpBroadcast;


fn main() {
    env_logger::init().expect("env_logger initialization failed!");

    info!("logging initialized, starting up...");

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Intelligence>();

    info!("world created, components registered!");

    survivor::add_survivor(&mut world, None);
    zombie::add_zombie(&mut world, None);

    let mut dispatcher = DispatcherBuilder::new()
        .add(Movement, "movement", &[])
        .add(Printer, "printer", &[])
        .add(ZombieSpawner, "zombie_spawner", &[])
        // .add(TcpBroadcast, "tcp_broadcast", &[])
        .build();

    info!("dispatcher built, dispatching...");

    dispatcher.dispatch(&mut world.res);
    world.maintain();

    info!("simulation shutting down...");
}
