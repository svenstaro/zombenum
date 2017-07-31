use specs::{Component, BTreeStorage};


pub struct Range {
    pub melee: f32,
    pub throw: f32,
    pub shoot: f32,
}

impl Component for Range {
    type Storage = BTreeStorage<Self>;
}


pub struct Accuracy {
    pub melee: f32,
    pub throw: f32,
    pub shoot: f32,
}

impl Component for Accuracy {
    type Storage = BTreeStorage<Self>;
}


pub struct Durability {
    pub value: f32,
}

impl Component for Durability {
    type Storage = BTreeStorage<Self>;
}

impl Default for Durability {
    fn default() -> Durability {
        Durability {
            value: 1.0, // 100%
        }
    }
}


pub struct Damage {
    pub melee: u8,
    pub throw: u8,
    pub shoot: u8,
}

impl Component for Damage {
    type Storage = BTreeStorage<Self>;
}
