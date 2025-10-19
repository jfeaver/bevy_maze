use bevy::{color::Srgba, prelude::*, sprite::Anchor};

use crate::{
    AppSystems, PausableSystems,
    gameplay::{
        AtlasIndex, SpriteSheet,
        movement::MovementController,
        utils::{Hitbox, render_position_from_world_array_position},
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
    app.add_systems(OnEnter(Screen::Gameplay), spawn_player_anchor);
    app.add_systems(Update, update_player_anchor);
    app.add_systems(OnEnter(Screen::Gameplay), add_player);
}

fn add_player(mut commands: Commands, sheet: Res<SpriteSheet>) {
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
        Anchor::TOP_LEFT,
        Transform {
            translation: render_position_from_world_array_position(5.0, 1.0).extend(2.0),
            scale: Vec3::splat(crate::gameplay::SCALE_FACTOR),
            ..Default::default()
        },
        MovementController {
            hitbox: Some(Hitbox::new(Rect::from_corners(Vec2::ZERO, Vec2::ONE))),
            ..Default::default()
        },
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

#[derive(Component)]
struct PlayerAnchor;

fn spawn_player_anchor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create a small circle mesh
    commands.spawn((
        Name::new("Player Anchor"),
        PlayerAnchor,
        Mesh2d(meshes.add(Circle::new(0.1))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Srgba::new(1.0, 0.0, 0.0, 1.0)))),
        Transform::from_xyz(0.0, 4.0, 100.0),
    ));
}

fn update_player_anchor(
    player_query: Query<&Transform, With<Player>>,
    mut circle_query: Query<&mut Transform, (With<PlayerAnchor>, Without<Player>)>,
) {
    if let (Ok(player_tf), Ok(mut circle_tf)) = (player_query.single(), circle_query.single_mut()) {
        // Move the circle to match the player's position
        circle_tf.translation.x = player_tf.translation.x;
        circle_tf.translation.y = player_tf.translation.y;
    }
}
