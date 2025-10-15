use bevy::math::Vec2;

use crate::SCREEN_DIM;

pub fn coordinate_translation(x: usize, y: usize) -> Vec2 {
    let offset = (SCREEN_DIM - 1) as f32 / 2.0;
    Vec2::new(x as f32 - offset, -(y as f32 - offset))
}
