use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    gameplay::{
        SpriteSheet, TILE_DIM,
        environment::coordinate::Coordinate,
        movement::MovementController,
        player::Player,
        utils::{
            hitbox::Hitbox, render_position_from_world_array_position,
            world_array_position_from_render_position,
        },
    },
    screens::Screen,
    utils::Z,
};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<MazeProgress>();

    app.add_systems(OnEnter(Screen::Gameplay), (init_maze, spawn_finish_line));
    app.add_systems(OnEnter(MazeProgress::Start), start_seeking); // TODO: Add some intro dialogue
    // app.add_systems(OnEnter(MazeProgress::Finish), win_dialogue); // TODO: Add some finish dialogue
    app.add_systems(
        Update,
        detect_finish_line_crossing
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
enum MazeProgress {
    #[default]
    None,
    Start,
    Seeking,
    Finish,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct FinishLine {
    hitbox: Hitbox,
}

fn init_maze(mut next_maze_progress: ResMut<NextState<MazeProgress>>) {
    next_maze_progress.set(MazeProgress::Start)
}

fn start_seeking(mut next_maze_progress: ResMut<NextState<MazeProgress>>) {
    next_maze_progress.set(MazeProgress::Seeking)
}

fn spawn_finish_line(mut commands: Commands, sheet: Res<SpriteSheet>) {
    // from coordinate 10, 1 to coordinate 10, 2 and covering those tiles (+1, +1)
    let corners = (
        Coordinate::new(10, 1).to_vec2(),
        Coordinate::new(10 + 1, 2 + 1).to_vec2(),
    );
    commands.spawn((
        Name::new("Finish Line"),
        FinishLine {
            hitbox: Hitbox::from_corners(corners.0, corners.1),
        },
        Sprite {
            image: sheet.texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: sheet.layout.clone(),
                index: 5,
            }),
            custom_size: Some(Vec2::new(1.0, 2.0) / crate::gameplay::SCALE_FACTOR),
            image_mode: SpriteImageMode::Tiled {
                tile_x: false,
                tile_y: true,
                stretch_value: 1.0,
            },
            ..Default::default()
        },
        Transform {
            translation: render_position_from_world_array_position(
                (corners.0.x + corners.1.x) / 2.0 * TILE_DIM,
                (corners.0.y + corners.1.y) / 2.0 * TILE_DIM,
            )
            .extend(Z.interactive),
            scale: Vec3::splat(crate::gameplay::SCALE_FACTOR),
            ..Default::default()
        },
        DespawnOnExit(Screen::Gameplay),
    ));
}

fn detect_finish_line_crossing(
    finish_line_query: Query<&FinishLine>,
    player_query: Query<(&MovementController, &Transform), With<Player>>,
    mut next_maze_progress: ResMut<NextState<MazeProgress>>,
) {
    if let (Ok(finish_line), Ok((controller, transform))) =
        (finish_line_query.single(), player_query.single())
    {
        // TODO: Shared code with movement?
        let player_position = world_array_position_from_render_position(
            transform.translation.x,
            transform.translation.y,
        );
        // TODO: If a movement controller only sometimes has a girth, shouldn't it just be a different component?
        if let Some(player_girth) = controller.girth {
            let half_girth = player_girth / 2.0;
            let player_hitbox = Hitbox::from_rounded_corners(
                player_position - half_girth,
                player_position + half_girth,
            );
            if finish_line.hitbox.contains_hitbox(player_hitbox) {
                // do stuff
                debug!("YOU WIN!");
                next_maze_progress.set(MazeProgress::Finish);
            }
        }
    }
}
