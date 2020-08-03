
#[derive(Clone, Copy)]
pub struct EngineConfig {
    pub debug: bool,
    pub time: f32,
    pub deep_frac: f32,
    pub deep_depth: isize,
}

impl EngineConfig {
    pub fn new(time: f32, deep_frac: f32, deep_depth: isize, debug: bool) -> Self {
        EngineConfig{
            time,
            deep_frac,
            deep_depth,
            debug,
        }
    }

    pub fn default() -> Self {
        Self::new(10f32, 0.5f32, 3, false)
    }

    pub fn default_debug(debug: bool) -> Self {
        Self::new(20f32, 0.5f32, 3, debug)
    }
}