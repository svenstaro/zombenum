use specs::Entity;
use specs::LazyUpdate;
use components::common::*;
use components::usable::*;


struct Pistol {
    range: Range,
    accuracy: Accuracy,
    durability: Durability,
    damage: Damage,
}

impl Pistol {
    /// Consume `self` and spawn an equivalent entity in the `World`.
    pub fn spawn(self, entity: Entitiy, lazy: &LazyUpdate) {
        lazy.insert(entity, self.range);
        lazy.insert(entity, self.accuracy);
        lazy.insert(entity, self.durability);
        lazy.insert(entity, self.damage);
    }
}
