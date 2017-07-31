use specs::Entities;
use specs::Fetch;
use specs::LazyUpdate;
use specs::System;
use entities::{survivor, zombie};
use entities::weapons::guns::Pistol;


pub struct SurvivorSpawner;

impl<'a> System<'a> for SurvivorSpawner {
    type SystemData = (Entities<'a>, Fetch<'a, LazyUpdate>);

    fn run(&mut self, (ent, lazy): Self::SystemData) {
        survivor::spawn_survivor(ent.create(), &lazy, None);
    }
}


pub struct ZombieSpawner;

impl<'a> System<'a> for ZombieSpawner {
    type SystemData = (Entities<'a>, Fetch<'a, LazyUpdate>);

    fn run(&mut self, (ent, lazy): Self::SystemData) {
        zombie::spawn_zombie(ent.create(), &lazy, None);
    }
}


pub struct PistolSpawner;

impl<'a> System<'a> for PistolSpawner {
    type SystemData = (Entities<'a>, Fetch<'a, LazyUpdate>);

    fn run(&mut self, (ent, lazy): Self::SystemData) {
        let pistol: Pistol = Default::default();
        pistol.spawn(ent.create(), &lazy);
    }
}
