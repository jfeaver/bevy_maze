use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    gameplay::{
        HALF_TILE_DIM,
        environment::{WorldMap, coordinate::Coordinate},
        utils::{
            flipped_y_axis, hitbox::Hitbox, render_x_from_world_array_x,
            render_y_from_world_array_y, world_array_position_from_render_position,
        },
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        apply_movement
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

/// These are the movement parameters for our character controller.
/// For now, this is only used for a single player, but it could power NPCs or
/// other players as well.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    /// The direction the entity wants to move in.
    pub intent: Vec2,

    /// Maximum speed in world units per second.
    /// 1 world unit = 1 pixel when using the default 2D camera and no physics engine.
    pub max_speed: f32,

    /// How large is the entity's hitbox for collisions?
    pub girth: Option<Vec2>,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            // three tiles per second is a nice default, but we can still vary this per character.
            max_speed: 3.0,
            girth: None,
        }
    }
}

fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&mut MovementController, &mut Transform)>,
    world_map: Res<WorldMap>,
) {
    for (controller, mut transform) in &mut movement_query {
        if controller.intent == Vec2::ZERO {
            return;
        }
        let velocity = controller.max_speed * controller.intent;
        let translation = velocity.extend(0.0) * time.delta_secs(); // world array space
        let position = world_array_position_from_render_position(
            transform.translation.x,
            transform.translation.y,
        );
        debug!("Position: {:?}", position);
        if let Some(girth) = controller.girth {
            let half_girth = girth / 2.0;
            let mut hitbox =
                Hitbox::from_rounded_corners(position - half_girth, position + half_girth);
            // Try to move by x
            if translation.x > 0.0 {
                debug!("x > 0");
                let x_translation = translation.truncate() * Vec2::X;
                // The moving entities hitbox
                hitbox.translate(x_translation);
                // Find world map obstructions
                // FIXME: This only checks the nearest tile to the extents of the object but should really check each neighboring tile along the girth (whether that is one tile or more than two).
                let coordinate = Coordinate::from(
                    position + Vec2::ZERO.with_x(half_girth.x) - Vec2::ZERO.with_y(half_girth.y)
                        + Vec2::ZERO.with_x(HALF_TILE_DIM),
                );
                let coordinate2 = Coordinate::from(
                    position
                        + Vec2::ZERO.with_x(half_girth.x)
                        + Vec2::ZERO.with_y(half_girth.y)
                        + Vec2::ZERO.with_x(HALF_TILE_DIM),
                );
                if let Some(tile) = world_map.at(coordinate)
                    && let Some(tile2) = world_map.at(coordinate2)
                {
                    let tile_obstruction = tile.is_obstruction();
                    let tile2_obstruction = tile2.is_obstruction();
                    if !(tile_obstruction || tile2_obstruction) {
                        transform.translation.x += translation.x;
                    } else {
                        let tile_hitbox = tile.hitbox(coordinate);
                        let tile2_hitbox = tile2.hitbox(coordinate2);
                        if (tile_obstruction && hitbox.intersects(&tile_hitbox))
                            || (tile2_obstruction && hitbox.intersects(&tile2_hitbox))
                        {
                            transform.translation.x =
                                render_x_from_world_array_x(tile_hitbox.x1() - half_girth.x);
                        } else {
                            transform.translation.x += translation.x;
                        }
                    }
                }
            } else if translation.x < 0.0 {
                debug!("x < 0");
                let x_translation = translation.truncate() * Vec2::X;
                // The moving entities hitbox
                hitbox.translate(x_translation);
                // Find world map obstructions
                let coordinate = Coordinate::from(
                    position
                        - Vec2::ZERO.with_x(half_girth.x)
                        - Vec2::ZERO.with_y(half_girth.y)
                        - Vec2::ZERO.with_x(HALF_TILE_DIM),
                );
                let coordinate2 = Coordinate::from(
                    position - Vec2::ZERO.with_x(half_girth.x) + Vec2::ZERO.with_y(half_girth.y)
                        - Vec2::ZERO.with_x(HALF_TILE_DIM),
                );
                if let Some(tile) = world_map.at(coordinate)
                    && let Some(tile2) = world_map.at(coordinate2)
                {
                    let tile_obstruction = tile.is_obstruction();
                    let tile2_obstruction = tile2.is_obstruction();
                    if !(tile_obstruction || tile2_obstruction) {
                        transform.translation.x += translation.x;
                    } else {
                        let tile_hitbox = tile.hitbox(coordinate);
                        let tile2_hitbox = tile2.hitbox(coordinate2);
                        if (tile_obstruction && hitbox.intersects(&tile_hitbox))
                            || (tile2_obstruction && hitbox.intersects(&tile2_hitbox))
                        {
                            transform.translation.x =
                                render_x_from_world_array_x(tile_hitbox.x2() + half_girth.x);
                        } else {
                            transform.translation.x += translation.x;
                        }
                    }
                }
            }
            // Reset hitbox in case movement didn't proceed fully in x direction
            // TODO: Optimization opportunity?
            let position = world_array_position_from_render_position(
                transform.translation.x,
                transform.translation.y,
            );
            hitbox = Hitbox::from_rounded_corners(position - half_girth, position + half_girth);
            // Try to move by y
            if translation.y > 0.0 {
                debug!("y > 0");
                let y_translation = translation.truncate() * Vec2::Y;
                // The moving entities hitbox
                hitbox.translate(y_translation);
                // Find world map obstructions
                let coordinate = Coordinate::from(
                    position + Vec2::ZERO.with_y(half_girth.y) - Vec2::ZERO.with_x(half_girth.x)
                        + Vec2::ZERO.with_y(HALF_TILE_DIM),
                );
                let coordinate2 = Coordinate::from(
                    position
                        + Vec2::ZERO.with_y(half_girth.y)
                        + Vec2::ZERO.with_x(half_girth.x)
                        + Vec2::ZERO.with_y(HALF_TILE_DIM),
                );
                if let Some(tile) = world_map.at(coordinate)
                    && let Some(tile2) = world_map.at(coordinate2)
                {
                    let tile_obstruction = tile.is_obstruction();
                    let tile2_obstruction = tile2.is_obstruction();
                    if !(tile_obstruction || tile2_obstruction) {
                        transform.translation.y += flipped_y_axis(translation.y);
                    } else {
                        let tile_hitbox = tile.hitbox(coordinate);
                        let tile2_hitbox = tile2.hitbox(coordinate2);
                        if (tile_obstruction && hitbox.intersects(&tile_hitbox))
                            || (tile2_obstruction && hitbox.intersects(&tile2_hitbox))
                        {
                            transform.translation.y =
                                render_y_from_world_array_y(tile_hitbox.y1() - half_girth.y);
                        } else {
                            transform.translation.y += flipped_y_axis(translation.y);
                        }
                    }
                }
            } else if translation.y < 0.0 {
                debug!("y < 0");
                let y_translation = translation.truncate() * Vec2::Y;
                // The moving entities hitbox
                hitbox.translate(y_translation);
                // Find world map obstructions
                let coordinate = Coordinate::from(
                    position
                        - Vec2::ZERO.with_y(half_girth.y)
                        - Vec2::ZERO.with_x(half_girth.x)
                        - Vec2::ZERO.with_y(HALF_TILE_DIM),
                );
                let coordinate2 = Coordinate::from(
                    position - Vec2::ZERO.with_y(half_girth.y) + Vec2::ZERO.with_x(half_girth.x)
                        - Vec2::ZERO.with_y(HALF_TILE_DIM),
                );
                if let Some(tile) = world_map.at(coordinate)
                    && let Some(tile2) = world_map.at(coordinate2)
                {
                    let tile_obstruction = tile.is_obstruction();
                    let tile2_obstruction = tile2.is_obstruction();
                    if !(tile_obstruction || tile2_obstruction) {
                        transform.translation.y += flipped_y_axis(translation.y);
                    } else {
                        let tile_hitbox = tile.hitbox(coordinate);
                        let tile2_hitbox = tile2.hitbox(coordinate2);
                        if (tile_obstruction && hitbox.intersects(&tile_hitbox))
                            || (tile2_obstruction && hitbox.intersects(&tile2_hitbox))
                        {
                            transform.translation.y =
                                render_y_from_world_array_y(tile_hitbox.y2() + half_girth.y);
                        } else {
                            transform.translation.y += flipped_y_axis(translation.y);
                        }
                    }
                }
            }
        } else {
            todo!("Non-Hitbox movement is not implemented.")
        }
        debug!("OK")
    }
}
