use bevy::prelude::*;

use crate::components::body::Body;
use crate::utility::utility_funcs::*;
use crate::components::{SysCamera, CameraState, CameraFocusType};

/// Updates the camera position and scale based on the current camera focus type
///
/// This system handles different camera behaviors including global view,
/// body-centered view, and fixed positions with various scaling options.
pub fn update_camera_position(
    query_window: Query<&Window>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<SysCamera>>,
    camera_state: Res<CameraState>,
    body_query: Query<(Entity, &Body)>
) {
    // Get camera and update fields
    let (mut transform, mut projection) = camera_query.single_mut();

    // Get new center / scale
    match &camera_state.focus_type {
        CameraFocusType::Global(margin) => {
            let bodies = extract_bodies(&body_query);
            let (position, scale) = get_camera_global_settings(bodies, query_window, *margin);
            update_position(&mut transform, &position);
            projection.scale = scale;
        },
        CameraFocusType::BodyCentered(id, scale) => {
            // Center position on the body if it was found
            if let Ok(body) = body_query.get_component::<Body>(Entity::from_raw(*id)) {
                update_position(&mut transform, body.get_position());
            }

            // only update if use changed the scale value
            if projection.scale != *scale {
                projection.scale = *scale;
            }
        },
        CameraFocusType::Fixed(position, scale) => {
            // Update center
            update_position(&mut transform, position);

            // only update if use changed the scale value
            if projection.scale != *scale {
                projection.scale = *scale;
            }
        },
        CameraFocusType::FixedAutoScale(position, margin) => {
            let bodies = extract_bodies(&body_query);
            let scale = get_camera_fixed_settings(*position, bodies, query_window, *margin);
            update_position(&mut transform, position);
            projection.scale = scale;
        },
        CameraFocusType::FixedMaxAutoScale(position, margin) => {
            let bodies = extract_bodies(&body_query);
            let scale = get_camera_fixed_settings(*position, bodies, query_window, *margin);
            update_position(&mut transform, position);

            // only update if scale is larger than current scale
            if projection.scale < scale {
                projection.scale = scale;
            }
        }
    };
}

/// Calculates camera position and scale to view all bodies with margin
///
/// Determines optimal camera settings to ensure all bodies are visible
/// within the window bounds with the specified margin.
///
/// # Parameters
/// * `bodies` - Collection of bodies to include in the view
/// * `query_window` - Window query for screen dimensions
/// * `margin` - Extra space around the bounds as a percentage
///
/// # Returns
/// A tuple containing:
/// * `Vec2` - The optimal center position for the camera
/// * `f32` - The optimal scale factor for the projection
fn get_camera_global_settings(bodies: Vec<&Body>, query_window: Query<&Window>, margin: f32) -> (Vec2, f32){
    let window = query_window.single();
    let bounds = get_window_bounds(&bodies);
    get_camera_setting_on_bounds(
        bounds,
        window.width(),
        window.height(),
        margin)
}

/// Calculates camera scale for a fixed position to view all bodies
///
/// Determines the appropriate scale to ensure all bodies are visible
/// when the camera is fixed at the specified center position.
///
/// # Parameters
/// * `center` - Fixed center position of the camera
/// * `bodies` - Collection of bodies to include in the view
/// * `query_window` - Window query for screen dimensions
/// * `margin` - Extra space around the bounds as a percentage
///
/// # Returns
/// * `f32` - The optimal scale factor for the projection
fn get_camera_fixed_settings(center: Vec2, bodies: Vec<&Body>, query_window: Query<&Window>, margin: f32) -> f32 {
    let window = query_window.single();
    let bounds = get_window_bounds(&bodies);
    get_camera_settings_on_center(
        bounds,
        center,
        window.width(),
        window.height(),
        margin)
}

/// Extracts body components from an entity query
///
/// Collects all Body components into a Vec for easier processing
///
/// # Parameters
/// * `body_query` - Query containing entity and body component pairs
///
/// # Returns
/// * `Vec<&'a Body>` - A collection of references to Body components
fn extract_bodies<'a>(body_query: &'a Query<(Entity, &Body)>) -> Vec<&'a Body> {
    body_query.iter().map(|t| t.1).collect()
}

/// Updates a transform's position while preserving the z-coordinate
///
/// Sets the x and y components of the transform's translation
/// to match the provided 2D position vector
///
/// # Parameters
/// * `transform` - Transform to be updated
/// * `position` - New 2D position
fn update_position(transform: &mut Transform, position: &Vec2) {
    transform.translation.x = position.x;
    transform.translation.y = position.y;
}