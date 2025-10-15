// The meat and potatoes module. Where the fun stuff lives.

use crate::{gameplay::environment::WorldMap, screens::Screen};
use bevy::prelude::*;

mod environment;

const TILE_DIM: f32 = 1.0;
const PIXELS_PER_TILE: f32 = 16.0;
const SCALE_FACTOR: f32 = 1.0 / PIXELS_PER_TILE;

#[derive(Resource)]
pub struct SpriteSheet {
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
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

// A system that spawns the static world elements around the player
pub(crate) fn spawn_environment(
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
            for (translation, sprite_index) in
                crate::gameplay::environment::local_environment_objects(&world_map)
            {
                env.spawn(sprite_bundle(&sheet, sprite_index, translation));
            }
        });
}

fn sprite_bundle(sheet: &Res<SpriteSheet>, atlas_index: usize, translation: Vec3) -> impl Bundle {
    (
        Sprite::from_atlas_image(
            sheet.texture.clone(),
            TextureAtlas {
                layout: sheet.layout.clone(),
                index: atlas_index,
            },
        ),
        Transform {
            translation,
            scale: Vec3::splat(crate::gameplay::SCALE_FACTOR),
            ..Default::default()
        },
    )
}
