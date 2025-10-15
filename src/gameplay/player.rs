use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    gameplay::{
        AtlasIndex, SpriteSheet, movement::MovementController, sprite_bundle,
        utils::coordinate_translation,
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
    app.add_systems(OnEnter(Screen::Gameplay), add_player);
}

fn add_player(mut commands: Commands, sheet: Res<SpriteSheet>) {
    let (sprite, transform): (Sprite, Transform) = sprite_bundle(
        &sheet,
        Player.atlas_index().unwrap_or(0),
        coordinate_translation(5, 1).extend(2.0),
    );

    commands.spawn((
        Name::new("Player"),
        Player,
        sprite,
        transform,
        MovementController::default(),
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

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
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
