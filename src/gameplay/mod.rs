// The meat and potatoes module. Where the fun stuff lives.

use crate::{PIXELS_PER_TILE, gameplay::environment::WorldMap};
use bevy::{asset::RenderAssetUsages, prelude::*};

mod environment;
mod movement;
mod player;
mod utils;

const SCALE_FACTOR: f32 = 1.0 / PIXELS_PER_TILE as f32;
const ATLAS_COLS: usize = 15;
const ATLAS_ROWS: usize = 15;

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
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let mut i: u32 = 0;
    for (translation, sprite_index) in
        crate::gameplay::environment::local_environment_objects(&world_map)
    {
        let tile_size = 1.0;
        let offset_x = translation.x - tile_size / 2.0;
        let offset_y = translation.y - tile_size / 2.0;

        let u = (sprite_index % ATLAS_COLS) as f32 / ATLAS_COLS as f32;
        let v = (sprite_index / ATLAS_COLS) as f32 / ATLAS_ROWS as f32;
        let du = 1.0 / ATLAS_COLS as f32;
        let dv = 1.0 / ATLAS_ROWS as f32;

        // quad positions
        positions.extend([
            [offset_x, offset_y, 0.0],
            [offset_x + tile_size, offset_y, 0.0],
            [offset_x + tile_size, offset_y + tile_size, 0.0],
            [offset_x, offset_y + tile_size, 0.0],
        ]);

        // UVs into the atlas
        uvs.extend([[u, v + dv], [u + du, v + dv], [u + du, v], [u, v]]);

        indices.extend([i, i + 1, i + 2, i, i + 2, i + 3]);
        i += 4;
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(bevy::mesh::Indices::U32(indices));

    let mesh_handle = meshes.add(mesh);

    let material = materials.add(ColorMaterial::from(sheet.texture.clone()));

    commands.spawn((
        Name::new("Environment"),
        Mesh2d(mesh_handle),
        MeshMaterial2d(material),
    ));
}
