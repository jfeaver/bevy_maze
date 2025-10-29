// The meat and potatoes module. Where the fun stuff lives.

use crate::{
    PIXELS_PER_TILE,
    asset_tracking::LoadResource,
    gameplay::environment::WorldMap,
    screens::Screen,
    utils::{
        Z,
        tile_mesh::{AtlasConfig, build_tile_mesh},
    },
};
use bevy::prelude::*;

mod animation;
mod environment;
mod maze;
mod movement;
mod player;
mod utils;

const TILE_DIM: f32 = 1.0;
const SCALE_FACTOR: f32 = TILE_DIM / PIXELS_PER_TILE as f32;
const ATLAS_COLS: u16 = 15;
const ATLAS_ROWS: u16 = 15;

#[derive(Resource, Asset, Clone, Reflect)]
pub struct SpriteSheet {
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<WorldMap>();
    app.load_resource::<SpriteSheet>();
    app.add_plugins((
        animation::plugin,
        player::plugin,
        maze::plugin,
        movement::plugin,
    ));
}

impl FromWorld for SpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture = { world.resource::<AssetServer>().load("textures/tileset.png") };
        let mut atlases = world.resource_mut::<Assets<TextureAtlasLayout>>();
        Self {
            texture,
            layout: atlases.add(TextureAtlasLayout::from_grid(
                UVec2::splat(16),
                15,
                15,
                None,
                None,
            )),
        }
    }
}

// A system that spawns the static world elements around the player
pub(crate) fn spawn_environment(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    sheet: Res<SpriteSheet>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tile_mesh = build_tile_mesh(
        crate::gameplay::environment::local_environment_objects(&world_map),
        &AtlasConfig {
            cols: ATLAS_COLS,
            rows: ATLAS_ROWS,
        },
        TILE_DIM,
        &mut meshes,
    );

    let material = materials.add(ColorMaterial::from(sheet.texture.clone()));

    commands.spawn((
        Name::new("Environment"),
        Mesh2d(tile_mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, -TILE_DIM, Z.ground), // Account for Bevy using Y-Up coordinates and us using Y-Down
        DespawnOnExit(Screen::Gameplay),
    ));
}
