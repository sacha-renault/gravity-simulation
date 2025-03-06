use bevy::prelude::*;

#[derive(Component)]
pub struct CameraScaling(pub f32);

#[derive(Component)]
pub struct SysCamera;

#[derive(Resource)]
pub struct CameraState { pub focus_type: CameraFocusType }

/// Defines how the camera should position and scale itself in the simulation
pub enum CameraFocusType {
    /// Camera is fixed at a specific position with a defined scale
    /// Parameters: (position: Vec2, scale: f32)
    Fixed(Vec2, f32),

    /// Camera position is fixed at a specific point, but scale is automatically calculated to keep all bodies in view
    /// Parameter: (position: Vec2, margin: f32) - The fixed position where the camera will remain centered
    FixedAutoScale(Vec2, f32),

    /// Camera is centered on a specific Body entity
    /// Parameters: (entity_id: u32, scale: f32)
    BodyCentered(u32, f32),

    /// Camera will automatically adjust to keep all bodies in view
    /// Parameter: (margin: f32) - Additional space around the bounds of all bodies
    Global(f32)
}