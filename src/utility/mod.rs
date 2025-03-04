pub mod consts;
pub mod types;
pub mod utility_funcs;

pub use {
    consts::{G, MASS_SCALE, DISTANCE_SCALE},
    types::Force,
    utility_funcs::*,
};