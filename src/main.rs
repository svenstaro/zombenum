extern crate specs;
extern crate ggez;
extern crate env_logger;
extern crate rand;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;


mod components;
mod systems;
mod resources;
mod entities;
mod util;


use specs::{World, RunNow, DispatcherBuilder};

use components::common::*;
use components::living::*;

use systems::{Movement, Printer, ZombieSpawner};
use systems::broadcast::TcpBroadcast;


fn main() {
    env_logger::init().expect("env_logger initialization failed!");

    info!("logging initialized, starting up...");

    let mut world = World::new();
    world.register::<Name>();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Health>();
    world.register::<Agility>();
    world.register::<Intelligence>();
    world.register::<Nourishment>();
    world.register::<Hunger>();
    world.register::<Thirst>();

    info!("world created, components registered!");

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

    println!("Generating a name to see if it works...");
    println!("{}", util::NameGenerator::gen_name());
}
