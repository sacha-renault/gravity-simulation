use bevy::prelude::*;

/// Defines how the camera should position and scale itself in the simulation
pub enum FocusType {
    /// Camera is fixed at a specific position with a defined scale
    Fixed,

    /// Camera position is fixed at a specific point, but scale is automatically calculated to keep all bodies in view
    FixedAutoScale,

    /// Camera position is fixed with scale that only increases automatically to fit all bodies
    /// Scale can only decrease through user input, preventing automatic zooming in when bodies move closer together
    FixedMaxAutoScale,

    /// Camera is centered on a specific Body entity
    BodyCentered(u32),

    /// Camera will automatically adjust to keep all bodies in view
    Global,
}

#[derive(Component)]
pub struct SysCamera;

#[derive(Resource)]
pub struct CameraState {
    focus_type: FocusType,
    position: Vec2,
    scale: f32,
    margin: f32,
}

impl CameraState {
    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn set_scale_by(&mut self, ratio: f32) {
        self.scale *= ratio;
    }

    pub fn margin(&self) -> f32 {
        self.margin
    }

    pub fn set_margin(&mut self, margin: f32) {
        self.margin = margin;
    }

    pub fn position(&self) -> &Vec2 {
        &self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn translate_position_by(&mut self, delta: Vec2) {
        self.position += delta;
    }

    pub fn focus_type(&self) -> &FocusType {
        &self.focus_type
    }

    pub fn set_focus(&mut self, focus_type: FocusType) {
        self.focus_type = focus_type;
    }
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            focus_type: FocusType::Fixed,
            position: Vec2::default(),
            scale: 1.,
            margin: 1.2
        }
    }
}