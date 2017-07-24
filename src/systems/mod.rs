use specs::Entities;
use specs::Fetch;
use specs::LazyUpdate;
use specs::System;
use specs::ReadStorage;
use specs::WriteStorage;
use components::common::{Position, Velocity};
use components::living::Intelligence;
use zombie;


pub mod broadcast;


pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (ReadStorage<'a, Velocity>,
                       WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        use specs::Join;

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x;
            pos.y += vel.y;
        }
    }
}


pub struct Printer;

impl<'a> System<'a> for Printer {
    type SystemData = (ReadStorage<'a, Intelligence>,
                       ReadStorage<'a, Position>);

    fn run(&mut self, (int, pos): Self::SystemData) {
        use specs::Join;

        for (int, pos) in (&int, &pos).join() {
            let label = match int.iq > 80 {
                true => "survivor",
                false => "zombie",
            };

            println!("{} entity at {:?};", label, pos);
        }
    }
}


pub struct ZombieSpawner;

impl<'a> System<'a> for ZombieSpawner {
    type SystemData = (Entities<'a>,
                       Fetch<'a, LazyUpdate>);

    fn run(&mut self, (ent, lazy): Self::SystemData) {
        zombie::spawn_zombie(ent.create(), &lazy, None);
        info!("zombie spawned!");
    }
}
