use std::ops::Neg;

use bevy::math::Vec2;

use crate::SCREEN_DIM;

pub mod hitbox;

// "Render position" here is Bevy's right handed, Y-Up position.
// (thumb is x, index is y, middle is z)
// https://bevy-cheatbook.github.io/fundamentals/coords.html
// The axis is centered in the center of the screen.
// NOTE Sprites are rendered from _ corner (?)

// "World array position" here is my left handed, Y-Down position.
// (thumb is x, index is y, middle is z)
// The axis is centered in the top left of the screen.

pub fn world_array_position_from_render_position(x: f32, y: f32) -> Vec2 {
    let offset = SCREEN_DIM as f32 / 2.0;
    // render position is to the right (positive x) and to the bottom (negative y in render space)
    Vec2::new(x + offset, flipped_y_axis(y) - flipped_y_axis(offset))
}

pub fn render_position_from_world_array_position(x: f32, y: f32) -> Vec2 {
    Vec2::new(
        render_x_from_world_array_x(x),
        render_y_from_world_array_y(y),
    )
}

pub fn render_x_from_world_array_x(x: f32) -> f32 {
    let offset = SCREEN_DIM as f32 / 2.0;
    // world array position is to the left (negative x)
    x - offset
}

pub fn render_y_from_world_array_y(y: f32) -> f32 {
    let offset = SCREEN_DIM as f32 / 2.0;
    // world array position is to the top (negative y in world array space)
    flipped_y_axis(y) - flipped_y_axis(offset)
}

/// A documentation as code function making it explicit where we have to account
/// for using a different coordinate system.
#[inline]
pub fn flipped_y_axis<T>(val: T) -> T
where
    T: Neg<Output = T>,
{
    -val
}

fn round_out_float_noise(n: f32) -> f32 {
    (n * 10000.0).round() / 10000.0
}
