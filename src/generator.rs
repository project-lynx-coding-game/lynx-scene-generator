use crate::Map;

#[derive(Debug)]
pub struct Generator {
    seed: u64
}

impl Generator {
    pub fn new(seed: u64) -> Self {
        Generator { seed}
    }

    pub fn generate(&self, map: &Map) {
        // TODO generation
    }
}