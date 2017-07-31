use specs::Entities;
use specs::{Fetch, FetchMut};
use specs::LazyUpdate;
use specs::System;
use specs::ReadStorage;
use specs::WriteStorage;

use components::common::{Position, Velocity};
use components::living::Intelligence;

use entities::zombie;

use resources::TickCounter;


pub mod broadcast;
pub mod spawn;


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


pub struct Ticker;

impl<'a> System<'a> for Ticker {
    type SystemData = FetchMut<'a, TickCounter>;

    fn run(&mut self, data: Self::SystemData) {
        let mut ticker = data;

        ticker.increment();

        trace!("{} ticks have passed.", ticker.ticks);
    }
}
