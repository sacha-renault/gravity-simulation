use bevy::prelude::*;

use crate::components::body::Body;

#[derive(Component)]
pub struct CameraScaling(pub f32);

#[derive(Component)]
pub struct SysCamera;

#[derive(Component)]
pub enum CameraFocusType {
    /// Camera is fixed on a point
    Fixed(Vec3),

    /// Camera is center on `Body`
    BodyCentered(u64),

    /// Camera will try to englobe all the bodies in on a single view
    Global(f32)
}