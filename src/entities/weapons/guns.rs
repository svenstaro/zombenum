use specs::Entity;
use specs::LazyUpdate;
use components::common::*;
use components::usable::*;


pub struct Pistol {
    pub range: Range,
    pub accuracy: Accuracy,
    pub durability: Durability,
    pub damage: Damage,
}

impl Pistol {
    /// Consume `self` and spawn an equivalent entity in the `World`.
    pub fn spawn(self, entity: Entity, lazy: &LazyUpdate) {
        lazy.insert(entity, self.range);
        lazy.insert(entity, self.accuracy);
        lazy.insert(entity, self.durability);
        lazy.insert(entity, self.damage);
    }
}

impl Default for Pistol {
    fn default() -> Pistol {
        let range = Range {
            melee: 1.0,
            throw: 2.0,
            shoot: 15.0,
        };

        let accuracy = Accuracy {
            melee: 80.0,
            throw: 50.0,
            shoot: 90.0,
        };

        let damage = Damage {
            melee: 5,
            throw: 5,
            shoot: 60,
        };

        Pistol {
            range: range,
            accuracy: accuracy,
            durability: Default::default(),
            damage: damage,
        }
    }
}
