use bevy::prelude::*;

#[derive(Component)]
pub struct CameraScaling(pub f32);

#[derive(Component)]
pub struct SysCamera;

#[derive(Resource)]
pub struct CameraState { pub focus_type: CameraFocusType }

pub enum CameraFocusType {
    /// Camera is fixed on a point
    Fixed(Vec2),

    /// Camera is center on `Body`
    BodyCentered(u32),

    /// Camera will try to englobe all the bodies in on a single view
    Global(f32)
}