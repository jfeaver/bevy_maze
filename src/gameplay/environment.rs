// draw the static environment

use bevy::prelude::*;
use coordinate::Coordinate;

use crate::{
    SCREEN_DIM as MAP_DIM,
    gameplay::{
        TILE_DIM,
        utils::{hitbox::Hitbox, render_position_from_world_array_position},
    },
};

pub mod coordinate;
mod world_map_array;

#[derive(Reflect, PartialEq, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Reflect, Debug)]
pub(in crate::gameplay) enum GroundType {
    Grass,
    DirtV,
    DirtH,
}

#[derive(Reflect, Debug, PartialEq)]
pub(in crate::gameplay) enum ObstructionType {
    None,
    WallV,
    WallH,
    Tower,
    Rock1,
    Rock2,
    Rock3,
}

impl GroundType {
    fn atlas_index(&self) -> Option<usize> {
        match self {
            GroundType::Grass => Some(2),
            GroundType::DirtH => Some(3),
            GroundType::DirtV => Some(4),
        }
    }
}

impl ObstructionType {
    fn atlas_index(&self) -> Option<usize> {
        match self {
            ObstructionType::None => None,
            ObstructionType::WallV => Some(31),
            ObstructionType::WallH => Some(30),
            ObstructionType::Tower => Some(32),
            ObstructionType::Rock1 => Some(15),
            ObstructionType::Rock2 => Some(16),
            ObstructionType::Rock3 => Some(17),
        }
    }
}

#[derive(Reflect, Debug)]
pub(crate) struct Tile {
    ground: GroundType,
    obstruction: ObstructionType,
}

impl Tile {
    pub fn is_obstruction(&self) -> bool {
        self.obstruction != ObstructionType::None
    }

    pub fn hitbox(&self, coordinate: Coordinate) -> Hitbox {
        Hitbox::from_corners(
            Vec2::from(coordinate),
            Vec2::from(coordinate) + Vec2::splat(TILE_DIM),
        )
    }
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub(crate) struct WorldMap {
    // The world is one screen square at the moment.
    pub(crate) grid: [[Tile; MAP_DIM as usize]; MAP_DIM as usize],
}

impl WorldMap {
    // Provide a position in world array coordinates
    pub fn at(&self, coordinate: Coordinate) -> Option<&Tile> {
        // Ensure coordinates are within grid bounds
        if coordinate.x < MAP_DIM as i32
            && coordinate.x >= 0
            && coordinate.y < MAP_DIM as i32
            && coordinate.y >= 0
        {
            Some(&self.grid[coordinate.y as usize][coordinate.x as usize])
        } else {
            None
        }
    }
}

impl Default for WorldMap {
    fn default() -> Self {
        Self {
            grid: world_map_array::TILE_MAP,
        }
    }
}

/// Returns an iterator over (translation, atlas_index_with_depth) for all visible environment tiles.
/// Each tile may contribute up to two sprites: ground (z=0.0) and obstruction (z=1.0).
pub(super) fn local_environment_objects(world_map: &WorldMap) -> impl Iterator<Item = (Vec2, u16)> {
    world_map
        .grid
        .iter()
        .enumerate()
        .flat_map(|(row, row_tiles)| {
            row_tiles
                .iter()
                .enumerate()
                .flat_map(move |(column, tile)| {
                    let base_translation =
                        render_position_from_world_array_position(column as f32, row as f32);

                    let mut entries = Vec::new();

                    if let Some(index) = tile.ground.atlas_index() {
                        entries.push((base_translation, index as u16));
                    }

                    if let Some(index) = tile.obstruction.atlas_index() {
                        entries.push((base_translation, index as u16));
                    }

                    entries
                })
        })
}
