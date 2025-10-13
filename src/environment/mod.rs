// draw the walls of the maze

use bevy::{color::palettes::css::RED, prelude::*};

use crate::screens::Screen;

const TILE_DIM: f32 = 1.0;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<WorldMap>();
}

#[derive(Reflect, Debug)]
enum TileType {
    Floor,
    Wall,
}

#[derive(Reflect, Debug)]
struct Tile {
    typ: TileType,
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct WorldMap {
    grid: Vec<Vec<Tile>>,
}

impl Default for WorldMap {
    fn default() -> Self {
        Self {
            grid: Vec::from([Vec::from([Tile {
                typ: TileType::Wall,
            }])]),
        }
    }
}

// A system that spawns the walls of the maze
pub fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // world_map: Res<WorldMap>,
) {
    commands.spawn((
        Name::new("Walls"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![(
            Mesh2d(meshes.add(Rectangle::new(TILE_DIM, TILE_DIM))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
            Transform::from_translation(Vec2::new(0.0, 0.0).extend(1.0)),
        )],
    ));
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
