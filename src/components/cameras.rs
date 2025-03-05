use bevy::prelude::*;

use crate::components::body::Body;

#[derive(Component)]
pub struct CameraScaling(f32);

#[derive(Component)]
pub struct SysCamera;

#[derive(Component)]
pub enum CameraFocusType<'a> {
    /// Camera is fixed on a point
    Fixed(Vec3),

    /// Camera is center on `Body`
    BodyCentered(&'a Body),

    /// Camera will try to englobe all the bodies in on a single view
    Global(f32)
}