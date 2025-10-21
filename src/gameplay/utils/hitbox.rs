use bevy::{
    ecs::component::Component,
    math::{Rect, Vec2},
    reflect::Reflect,
};

/// A simple hitbox wrapper around `bevy::math::Rect<f32>` that exposes
/// convenient x1/y1/x2/y2 getters (and a few helpers).
#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct Hitbox {
    rect: Rect,
}

impl Hitbox {
    /// Create a new hitbox from a `Rect<f32>`.
    pub fn new(rect: Rect) -> Self {
        assert!(
            !rect.is_empty(),
            "An empty (or negative) Rect was used as to initialize a hitbox!"
        );
        Self { rect }
    }

    pub fn from_corners(p0: Vec2, p1: Vec2) -> Self {
        Self {
            rect: Rect::from_corners(p0, p1),
        }
    }

    /// Left / minimum x
    pub fn x1(&self) -> f32 {
        self.rect.min.x
    }

    /// Bottom / minimum y
    pub fn y1(&self) -> f32 {
        self.rect.min.y
    }

    /// Right / maximum x
    pub fn x2(&self) -> f32 {
        self.rect.max.x
    }

    /// Top / maximum y
    pub fn y2(&self) -> f32 {
        self.rect.max.y
    }

    /// Width of the hitbox
    pub fn width(&self) -> f32 {
        self.x2() - self.x1()
    }

    /// Height of the hitbox
    pub fn height(&self) -> f32 {
        self.y2() - self.y1()
    }

    /// Move the hitbox by `delta`.
    pub fn translate(&mut self, delta: Vec2) {
        let min = self.rect.min + delta;
        let max = self.rect.max + delta;
        self.rect = Rect::from_corners(min, max);
    }

    /// Check whether a point is inside (inclusive) the hitbox.
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.x1() && point.x <= self.x2() && point.y >= self.y1() && point.y <= self.y2()
    }

    /// AABB intersection test with another hitbox.
    pub fn intersects(&self, other: &Hitbox) -> bool {
        !(self.x2() < other.x1()
            || self.x1() > other.x2()
            || self.y2() < other.y1()
            || self.y1() > other.y2())
    }

    // /// Replace the underlying rect.
    // pub fn set_rect(&mut self, rect: Rect<f32>) {
    //     self.rect = rect;
    // }
}

impl From<Rect> for Hitbox {
    fn from(r: Rect) -> Self {
        Self::new(r)
    }
}

impl From<Hitbox> for Rect {
    fn from(h: Hitbox) -> Self {
        h.rect
    }
}
