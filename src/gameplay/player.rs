use bevy::{prelude::*, sprite::Anchor};

use crate::{
    AppSystems, PausableSystems,
    gameplay::{
        AtlasIndex, SpriteSheet, TILE_DIM, movement::MovementController,
        utils::render_position_from_world_array_position,
    },
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        record_player_directional_input
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
    app.add_systems(OnEnter(Screen::Gameplay), spawn_player);
}

fn spawn_player(mut commands: Commands, sheet: Res<SpriteSheet>) {
    commands.spawn((
        Name::new("Player"),
        Player,
        Sprite::from_atlas_image(
            sheet.texture.clone(),
            TextureAtlas {
                layout: sheet.layout.clone(),
                index: Player.atlas_index().unwrap_or(0),
            },
        ),
        Anchor::CENTER,
        Transform {
            translation: render_position_from_world_array_position(5.5 * TILE_DIM, 1.5 * TILE_DIM)
                .extend(2.0),
            scale: Vec3::splat(crate::gameplay::SCALE_FACTOR),
            ..Default::default()
        },
        MovementController {
            girth: Some(Vec2::ONE * 0.8 * TILE_DIM),
            ..Default::default()
        },
        DespawnOnExit(Screen::Gameplay),
    ));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Player;

impl AtlasIndex for Player {
    fn atlas_index(&self) -> Option<usize> {
        Some(105)
    }
}

// NOTE: This creates intent in world array coordinate space
fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // Normalize intent so that diagonal movement is the same speed as horizontal / vertical.
    // This should be omitted if the input comes from an analog stick instead.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}
