use specs::{System, ReadStorage, WriteStorage};
use components::{Position, Velocity, Label};


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
    type SystemData = (ReadStorage<'a, Label>,
                       ReadStorage<'a, Position>);

    fn run(&mut self, (label, pos): Self::SystemData) {
        use specs::Join;

        for (label, pos) in (&label, &pos).join() {
            println!("Entity with label {:?} at position {:?};", label, pos);
        }
    }
}
