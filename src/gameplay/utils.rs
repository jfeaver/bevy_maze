use bevy::{
    ecs::component::Component,
    math::{Rect, Vec2},
    reflect::Reflect,
};

use crate::SCREEN_DIM;

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
    Vec2::new(x + offset, -(y - offset))
}

pub fn render_position_from_world_array_position(x: f32, y: f32) -> Vec2 {
    let offset = SCREEN_DIM as f32 / 2.0;
    Vec2::new(x - offset, -(y - offset))
}

// A simple rectangular hitbox component.
#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct Hitbox {
    pub rect: Rect,
}

impl Hitbox {
    pub fn new(rect: Rect) -> Self {
        Self { rect }
    }
}
