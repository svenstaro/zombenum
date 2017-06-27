extern crate specs;
extern crate ggez;

use specs::{Component, ReadStorage, System, VecStorage, World, RunNow};

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

impl Component for Position {
    type Storage = VecStorage<Position>;
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Velocity>;
}

struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

fn main() {
    let mut world = World::new();
    world.register::<Position>();

    world
        .create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .build();

    let mut hello_world = HelloWorld;
    hello_world.run_now(&world.res);
}
