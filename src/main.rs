extern crate specs;
extern crate ggez;
extern crate env_logger;
extern crate rand;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;


use std::time::{Duration, Instant};
use std::thread::sleep;

use specs::Component;
use specs::DispatcherBuilder;
use specs::ReadStorage;
use specs::System;
use specs::VecStorage;
use specs::World;
use specs::WriteStorage;
use specs::Fetch;
use specs::Join;

use components::common::*;
use components::living::*;

use systems::{Movement, Printer, Ticker};
use systems::spawn::{SurvivorSpawner, ZombieSpawner};
use systems::broadcast::TcpBroadcast;

use resources::TickCounter;


mod components;
mod systems;
mod resources;
mod entities;
mod util;


static TICKS_PER_SECOND: u64 = 60;


fn main() {
    env_logger::init().expect("env_logger initialization failed!");

    info!("logging initialized, starting up...");

    let mut world = World::new();

    world.add_resource(TickCounter { ticks: 0 });

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
        // .add(Printer, "printer", &[])
        .add(SurvivorSpawner, "survivor_spawner", &[])
        .add(ZombieSpawner, "zombie_spawner", &[])
        .add(Ticker, "ticker", &[])
        // .add(TcpBroadcast, "tcp_broadcast", &[])
        .build();

    let mut running = true;
    while running {
        let now = Instant::now();

        dispatcher.dispatch(&mut world.res);
        world.maintain();

        let max_tick_length = Duration::from_millis(1000 / TICKS_PER_SECOND);

        let remaining = match max_tick_length.checked_sub(now.elapsed()) {
            Some(duration) => duration,
            None => continue,
        };

        if remaining > Duration::from_millis(0) {
            sleep(remaining);
        }
    }

    info!("simulation shutting down...");
}
