// The meat and potatoes module. Where the fun stuff lives.

use crate::{
    PIXELS_PER_TILE,
    gameplay::environment::WorldMap,
    utils::tile_mesh::{AtlasConfig, build_tile_mesh},
};
use bevy::prelude::*;

mod environment;
mod movement;
mod player;
mod utils;

const SCALE_FACTOR: f32 = 1.0 / PIXELS_PER_TILE as f32;
const ATLAS_COLS: u16 = 15;
const ATLAS_ROWS: u16 = 15;

#[derive(Resource, Asset, Clone, Reflect)]
pub struct SpriteSheet {
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

trait AtlasIndex {
    fn atlas_index(&self) -> Option<usize>;
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<WorldMap>();
    app.init_resource::<SpriteSheet>();
    app.add_plugins((player::plugin, movement::plugin));
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
    let tile_size = 1.0;
    let tile_mesh = build_tile_mesh(
        crate::gameplay::environment::local_environment_objects(&world_map),
        &AtlasConfig {
            cols: ATLAS_COLS,
            rows: ATLAS_ROWS,
        },
        tile_size,
        &mut meshes,
    );

    let material = materials.add(ColorMaterial::from(sheet.texture.clone()));

    commands.spawn((
        Name::new("Environment"),
        Mesh2d(tile_mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, -1.0, 0.0), // Account for Bevy using Y-Up coordinates and us using Y-Down
    ));
}
