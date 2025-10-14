// draw the static environment

mod world_map_array;

use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use crate::screens::Screen;

const TILE_DIM: f32 = 1.0;
const PIXELS_PER_TILE: f32 = 16.0;
const SCALE_FACTOR: f32 = 1.0 / PIXELS_PER_TILE;

#[derive(Resource)]
pub struct SpriteSheet {
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

#[derive(Reflect, Debug)]
enum GroundType {
    Grass,
    DirtV,
    DirtH,
}

#[derive(Reflect, Debug)]
enum ObstructionType {
    None,
    WallV,
    WallH,
    Tower,
    Rock1,
    Rock2,
    Rock3,
}

trait AtlasIndex {
    fn atlas_index(&self) -> Option<usize>;
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
enum TileLayer {
    Grass,
    Wall,
}

#[derive(Reflect, Debug)]
struct Tile {
    ground: GroundType,
    obstruction: ObstructionType,
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct WorldMap {
    grid: [[Tile; 11]; 11],
}

impl Default for WorldMap {
    fn default() -> Self {
        Self {
            grid: world_map_array::TILE_MAP,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<WorldMap>();
    app.add_systems(Startup, load_assets);
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/tileset.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 15, 15, None, None);
    let layout_handle = atlases.add(layout);

    commands.insert_resource(SpriteSheet {
        texture,
        layout: layout_handle,
    });
}

fn spawn_sprite(
    env: &mut ChildSpawnerCommands,
    sheet: &Res<SpriteSheet>,
    atlas_index: Option<usize>,
    translation: Vec3,
) {
    match atlas_index {
        Some(index) => {
            env.spawn((
                Sprite::from_atlas_image(
                    sheet.texture.clone(),
                    TextureAtlas {
                        layout: sheet.layout.clone(),
                        index,
                    },
                ),
                Transform {
                    translation,
                    scale: Vec3::splat(SCALE_FACTOR),
                    ..Default::default()
                },
            ));
        }
        None => return,
    }
}

// A system that spawns the walls of the maze
pub fn spawn_environment(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    sheet: Res<SpriteSheet>,
) {
    commands
        .spawn((
            Name::new("Environment"),
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay),
        ))
        .with_children(|env| {
            for (row, row_tiles) in world_map.grid.iter().enumerate() {
                for (column, tile) in row_tiles.iter().enumerate() {
                    let translation = Vec2::new(
                        (column as f32 - 5.0) * TILE_DIM,
                        (-(row as f32) + 5.0) * TILE_DIM,
                    );
                    spawn_sprite(
                        env,
                        &sheet,
                        tile.ground.atlas_index(),
                        translation.extend(0.0),
                    );
                    spawn_sprite(
                        env,
                        &sheet,
                        tile.obstruction.atlas_index(),
                        translation.extend(1.0),
                    );
                }
            }
        });
}
