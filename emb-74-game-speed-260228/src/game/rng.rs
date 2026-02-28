use microbit::hal::Rng;

pub struct Prng {
    value: u32,
}

impl Prng {
    // Take a reference to the periphereal, so it still can be used by other parts
    pub fn seeded(hardware_rng: &mut Rng) -> Self {
        Self::new(hardware_rng.random_u32())
    }

    pub fn new(mut seed: u32) -> Self {
        if seed == 0 {seed = 1};
        Self { value: seed }
    }

    pub fn random_u32(&mut self) -> u32 {
        self.value = Self::xorshift32(self.value);
        self.value
    }

    fn xorshift32(mut x32: u32) -> u32 {
        x32 ^= x32 << 13;
        x32 ^= x32 >> 17;
        x32 ^= x32 << 5;
        x32
    }
}
