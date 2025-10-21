use bevy::math::Vec2;

/// A 2-dimensional grid coordinate in world array space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Convert this coordinate to a Bevy Vec2 (as floating-point values).
    pub fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    /// Convert from a Bevy Vec2 to a Coordinate by flooring the components.
    pub fn from_vec2_floor(v: Vec2) -> Self {
        Self {
            x: v.x.floor() as i32,
            y: v.y.floor() as i32,
        }
    }

    /// Convert from a Bevy Vec2 by rounding to the nearest integer.
    pub fn from_vec2_round(v: Vec2) -> Self {
        Self {
            x: v.x.round() as i32,
            y: v.y.round() as i32,
        }
    }

    /// Convert from a Bevy Vec2 by truncating (casting toward zero).
    pub fn from_vec2_trunc(v: Vec2) -> Self {
        Self {
            x: v.x as i32,
            y: v.y as i32,
        }
    }
}

// Common trait-based conversions (rounding strategy: floor)
impl From<Vec2> for Coordinate {
    fn from(v: Vec2) -> Self {
        Self::from_vec2_floor(v)
    }
}

impl From<Coordinate> for Vec2 {
    fn from(c: Coordinate) -> Self {
        c.to_vec2()
    }
}
