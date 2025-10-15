// draw the static environment

use bevy::prelude::*;

use crate::{
    SCREEN_DIM,
    gameplay::{AtlasIndex, utils::coordinate_translation},
};

mod world_map_array;

#[derive(Reflect, Debug)]
pub(in crate::gameplay) enum GroundType {
    Grass,
    DirtV,
    DirtH,
}

#[derive(Reflect, Debug)]
pub(in crate::gameplay) enum ObstructionType {
    None,
    WallV,
    WallH,
    Tower,
    Rock1,
    Rock2,
    Rock3,
}

impl AtlasIndex for GroundType {
    fn atlas_index(&self) -> Option<usize> {
        match self {
            GroundType::Grass => Some(2),
            GroundType::DirtH => Some(3),
            GroundType::DirtV => Some(4),
        }
    }
}

impl AtlasIndex for ObstructionType {
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

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub(crate) struct WorldMap {
    // The world is one screen square at the moment.
    pub(crate) grid: [[Tile; SCREEN_DIM as usize]; SCREEN_DIM as usize],
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
pub(super) fn local_environment_objects<'a>(
    world_map: &'a WorldMap,
) -> impl Iterator<Item = (Vec3, usize)> + 'a {
    world_map
        .grid
        .iter()
        .enumerate()
        .flat_map(|(row, row_tiles)| {
            row_tiles
                .iter()
                .enumerate()
                .flat_map(move |(column, tile)| {
                    let base_translation = coordinate_translation(column, row);

                    let mut entries = Vec::new();

                    if let Some(index) = tile.ground.atlas_index() {
                        entries.push((base_translation.extend(0.0), index));
                    }

                    if let Some(index) = tile.obstruction.atlas_index() {
                        entries.push((base_translation.extend(1.0), index));
                    }

                    entries
                })
        })
}
