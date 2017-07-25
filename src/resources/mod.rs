#[derive(Debug)]
pub struct DeltaTime {
    ticks: u64,
}

impl DeltaTime {
    pub fn new() -> DeltaTime {
        DeltaTime { ticks: 0, }
    }

    pub fn from<T: Into<u64>>(ticks: T) -> DeltaTime {
        DeltaTime {
            ticks: ticks.into(),
        }
    }

    pub fn get(&self) -> u64 {
        self.ticks
    }

    pub fn set<T: Into<u64>>(&mut self, ticks: T) {
        self.ticks = ticks.into();
    }

    pub fn increment(&mut self) {
        if self.ticks == u64::max_value() {
            // TODO
            // this is just here so we notice when an overflow happens
            // and we haven't dealt with that yet.
            panic!("DeltaTime overflow!");
        }

        self.ticks += 1;
    }
}
