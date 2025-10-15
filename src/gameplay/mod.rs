// The meat and potatoes module. Where the fun stuff lives.

use crate::{PIXELS_PER_TILE, gameplay::environment::WorldMap, screens::Screen};
use bevy::prelude::*;

mod environment;
mod movement;
mod player;
mod utils;

const SCALE_FACTOR: f32 = 1.0 / PIXELS_PER_TILE as f32;

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

fn sprite_bundle(
    sheet: &Res<SpriteSheet>,
    atlas_index: usize,
    translation: Vec3,
) -> (Sprite, Transform) {
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
