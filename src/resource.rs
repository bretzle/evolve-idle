pub struct Resource {
    pub amount: f32,
    pub max: f32,
    pub delta: f32,
    pub display: bool,
}

impl Resource {
    pub fn new(amount: f32, max: f32, display: bool) -> Self {
        Self {
            amount,
            max,
            delta: 0.0,
            display,
        }
    }

    pub fn is_full(&self) -> bool {
        self.amount >= self.max
    }
}

pub struct Resources {
    rna: Resource,
    dna: Resource,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            rna: Resource::new(0.0, 100.0, true),
            dna: Resource::new(0.0, 100.0, false),
        }
    }
}
