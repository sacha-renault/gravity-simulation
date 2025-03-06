use bevy::prelude::*;

use crate::components::body::Body;

pub fn get_window_bounds(bodies: &Vec<&Body>) -> (f32, f32, f32, f32) {
    let x_min = bodies
    .iter()
    .map(|body| body.get_position().x - body.get_radius())
    .fold(f32::INFINITY, |acc, x| acc.min(x));

    let x_max = bodies
        .iter()
        .map(|body| body.get_position().x + body.get_radius())
        .fold(f32::NEG_INFINITY, |acc, x| acc.max(x));

    let y_min = bodies
        .iter()
        .map(|body| body.get_position().y - body.get_radius())
        .fold(f32::INFINITY, |acc, y| acc.min(y));

    let y_max = bodies
        .iter()
        .map(|body| body.get_position().y + body.get_radius())
        .fold(f32::NEG_INFINITY, |acc, y| acc.max(y));

    (x_min, x_max, y_min, y_max)
}

pub fn get_camera_setting_on_bounds(
    bounds: (f32, f32, f32, f32),
    viewport_width: f32,
    viewport_height: f32,
    margin_factor: f32 // e.g., 1.2 for 20% padding
) -> (Vec2, f32) {
    let (x_min, x_max, y_min, y_max) = bounds;

    // Calculate current width and height of bounds
    let bounds_width = x_max - x_min;
    let bounds_height = y_max - y_min;

    // Find center point
    let center = Vec2::new(
        (x_min + x_max) / 2.0,
        (y_min + y_max) / 2.0
    );

    // For height to fit: scale must be at least bounds_height * padding_factor
    let scale_for_height= (bounds_height * margin_factor) / viewport_height;
    let scale_for_width = (bounds_width * margin_factor) / viewport_width;

    // Use the larger scale to ensure everything fits
    let scale = scale_for_height.max(scale_for_width);

    (center, scale)
}

pub fn get_camera_settings_on_center(
    bounds: (f32, f32, f32, f32),
    center: Vec2,
    viewport_width: f32,
    viewport_height: f32,
    margin_factor: f32 // e.g., 1.2 for 20% padding
) -> f32 {
    let (x_min, x_max, y_min, y_max) = bounds;
    let (x_center, y_center) = (center.x, center.y);
    let x_req = (x_center - x_min).abs().max((x_center - x_max).abs()) * 2.;
    let y_req = (y_center - y_min).abs().max((y_center - y_max).abs()) * 2.;

    // For height to fit: scale must be at least bounds_height * padding_factor
    let scale_for_height= (y_req * margin_factor) / viewport_height;
    let scale_for_width = (x_req * margin_factor) / viewport_width;

    // Use the larger scale to ensure everything fits
    scale_for_height.max(scale_for_width)
}