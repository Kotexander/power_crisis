pub struct Generator {
    feul: f32,
    efficiency: f32,
    running: bool,
}
impl Generator {
    pub fn new(feul: f32, efficiency: f32, running: bool) -> Self {
        Self {
            feul,
            efficiency,
            running,
        }
    }
    pub fn update(&mut self, delta: f32) {
        if self.running {
            self.feul -= self.efficiency * delta;
        }
    }

    /// Get the generator's feul
    pub fn feul(&self) -> f32 {
        self.feul
    }
    // /// Get the generator's efficiency
    // pub fn efficiency(&self) -> f32 {
    //     self.efficiency
    // }
    /// Returns wether the generator is running.
    pub fn running(&self) -> bool {
        self.running
    }

    /// Get a mutable reference to the generator's running.
    pub fn running_mut(&mut self) -> &mut bool {
        &mut self.running
    }

    /// Get a mutable reference to the generator's efficiency.
    pub fn efficiency_mut(&mut self) -> &mut f32 {
        &mut self.efficiency
    }
}
