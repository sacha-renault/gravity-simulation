// Scale factors
pub const MASS_SCALE: f32 = 1e24;
pub const DISTANCE_SCALE: f32 = 1e6;
pub const G: f32 = 6.6743e-11 * MASS_SCALE * (1.0 / (DISTANCE_SCALE * DISTANCE_SCALE * DISTANCE_SCALE));