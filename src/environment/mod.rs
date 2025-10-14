// draw the static environment

mod world_map_array;

use bevy::prelude::*;

use crate::screens::Screen;

const TILE_DIM: f32 = 1.0;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<WorldMap>();
}

// #[derive(Reflect, Debug)]
// enum TileType {
//     Floor,
//     Wall,
// }
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

// A system that spawns the walls of the maze
pub fn spawn_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    world_map: Res<WorldMap>,
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
                    // let hue: f32 = ((row as f32) * 15.0);
                    let hue: f32 = ((column as f32) + 3.0) * 5.0 + ((row as f32) * 15.0);
                    env.spawn((
                        Mesh2d(meshes.add(Rectangle::new(TILE_DIM, TILE_DIM))),
                        MeshMaterial2d(materials.add(ColorMaterial::from_color(
                            bevy::prelude::Color::oklch(0.7253, 0.1523, hue),
                        ))),
                        Transform::from_translation(
                            Vec2::new(
                                (column as f32 - 5.0) * TILE_DIM,
                                (-(row as f32) + 5.0) * TILE_DIM,
                            )
                            .extend(1.0),
                        ),
                    ));
                }
            }
            // env.spawn((
            //     Mesh2d(meshes.add(Rectangle::new(TILE_DIM, TILE_DIM))),
            //     MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
            //     Transform::from_translation(Vec2::new(0.0, 0.0).extend(1.0)),
            // ));
        });
}

// pub fn spawn_level(
//     mut commands: Commands,
//     level_assets: Res<LevelAssets>,
//     player_assets: Res<PlayerAssets>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
// ) {
//     commands.spawn((
//         Name::new("Level"),
//         Transform::default(),
//         Visibility::default(),
//         DespawnOnExit(Screen::Gameplay),
//         children![
//             player(400.0, &player_assets, &mut texture_atlas_layouts),
//             (
//                 Name::new("Gameplay Music"),
//                 music(level_assets.music.clone())
//             )
//         ],
//     ));
// }
