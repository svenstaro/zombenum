use specs::{System, ReadStorage, WriteStorage};
use components::common::{Position, Velocity};
use components::living::Intelligence;


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
