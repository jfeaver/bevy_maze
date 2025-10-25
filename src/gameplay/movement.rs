use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    AppSystems, PausableSystems,
    gameplay::{
        environment::{Direction, WorldMap, coordinate::Coordinate},
        utils::{
            hitbox::Hitbox, render_x_from_world_array_x, render_y_from_world_array_y,
            world_array_position_from_render_position,
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

fn hitbox_surroundings(hitbox: &Hitbox) -> (Coordinate, Coordinate) {
    let min_x = hitbox.x1().floor() as i32 - 1;
    let max_x = hitbox.x2().ceil() as i32;
    let min_y = hitbox.y1().floor() as i32 - 1;
    let max_y = hitbox.y2().ceil() as i32;
    (Coordinate::new(min_x, min_y), Coordinate::new(max_x, max_y))
}

fn gather_map_obstructions(
    surroundings: &(Coordinate, Coordinate),
    world_map: &Res<WorldMap>,
) -> HashMap<Coordinate, Option<Hitbox>> {
    let mut obstructions = HashMap::new();
    for x in surroundings.0.x..=surroundings.1.x {
        for y in surroundings.0.y..=surroundings.1.y {
            let coordinate = Coordinate::new(x, y);
            if let Some(tile) = world_map.at(coordinate) {
                if tile.is_obstruction() {
                    obstructions.insert(coordinate, Some(tile.hitbox(coordinate)));
                } else {
                    obstructions.insert(coordinate, None);
                }
            } else {
                obstructions.insert(coordinate, None);
            }
        }
    }
    obstructions
}

/// Returns the exact position in that direction that should be traveled to,
/// to avoid clipping into environment objects. Position returned is in world
/// array space.
fn apply_movement_in_one_direction(
    obstructions: &HashMap<Coordinate, Option<Hitbox>>,
    directional_translation: (Direction, f32),
    hitbox: &Hitbox,  // world array space
    half_girth: Vec2, // world array space
) -> f32 {
    let mut travel_to: f32;
    let collision_area: Hitbox; // Newly occupied space
    let use_gt: bool; // When deciding the furthest we should travel should we eliminate travel further from 0?
    let adjustment_fn: fn(Hitbox, Vec2) -> f32;

    match directional_translation {
        (Direction::North, y) => {
            travel_to = hitbox.y1() + y + half_girth.y;
            collision_area = Hitbox::from_rounded_corners(
                Vec2::new(hitbox.x1(), hitbox.y1() + y),
                Vec2::new(hitbox.x2(), hitbox.y1()),
            );
            use_gt = false;
            adjustment_fn = |obs_hitbox, half_girth: Vec2| obs_hitbox.y2() + half_girth.y;
        }
        (Direction::South, y) => {
            travel_to = hitbox.y2() + y - half_girth.y;
            collision_area = Hitbox::from_rounded_corners(
                Vec2::new(hitbox.x1(), hitbox.y2()),
                Vec2::new(hitbox.x2(), hitbox.y2() + y),
            );
            use_gt = true;
            adjustment_fn = |obs_hitbox, half_girth: Vec2| obs_hitbox.y1() - half_girth.y;
        }
        (Direction::East, x) => {
            travel_to = hitbox.x2() + x - half_girth.x;
            collision_area = Hitbox::from_rounded_corners(
                Vec2::new(hitbox.x2(), hitbox.y1()),
                Vec2::new(hitbox.x2() + x, hitbox.y2()),
            );
            use_gt = true;
            adjustment_fn = |obs_hitbox, half_girth: Vec2| obs_hitbox.x1() - half_girth.x;
        }
        (Direction::West, x) => {
            travel_to = hitbox.x1() + x + half_girth.x;
            collision_area = Hitbox::from_rounded_corners(
                Vec2::new(hitbox.x1() + x, hitbox.y1()),
                Vec2::new(hitbox.x1(), hitbox.y2()),
            );
            use_gt = false;
            adjustment_fn = |obs_hitbox, half_girth: Vec2| obs_hitbox.x2() + half_girth.x;
        }
    }
    let min = Coordinate::from_vec2_floor(collision_area.min());
    let max = Coordinate::from_vec2_floor(collision_area.max());
    for x in min.x..=max.x {
        for y in min.y..=max.y {
            if let Some(Some(obs_hitbox)) = obstructions.get(&Coordinate::new(x, y))
                && collision_area.intersects(obs_hitbox)
            {
                let adjusted_travel_to = adjustment_fn(*obs_hitbox, half_girth);
                if use_gt {
                    if travel_to > adjusted_travel_to {
                        travel_to = adjusted_travel_to;
                    }
                } else if travel_to < adjusted_travel_to {
                    travel_to = adjusted_travel_to;
                }
            }
        }
    }
    travel_to
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
            let surroundings = hitbox_surroundings(&hitbox);
            let mut obstructions = gather_map_obstructions(&surroundings, &world_map);
            // Try to move by x
            if translation.x > 0.0 {
                debug!("x > 0");
                let x_translation = apply_movement_in_one_direction(
                    &obstructions,
                    (Direction::East, translation.x),
                    &hitbox,
                    half_girth,
                );
                transform.translation.x = render_x_from_world_array_x(x_translation);
            } else if translation.x < 0.0 {
                debug!("x < 0");
                let x_translation = apply_movement_in_one_direction(
                    &obstructions,
                    (Direction::West, translation.x),
                    &hitbox,
                    half_girth,
                );
                transform.translation.x = render_x_from_world_array_x(x_translation);
            }
            // Reset hitbox in case movement didn't proceed fully in x direction
            // TODO: Optimization opportunity?
            let position = world_array_position_from_render_position(
                transform.translation.x,
                transform.translation.y,
            );
            hitbox = Hitbox::from_rounded_corners(position - half_girth, position + half_girth);
            let updated_surroundings = hitbox_surroundings(&hitbox);
            if updated_surroundings != surroundings {
                obstructions = gather_map_obstructions(&surroundings, &world_map);
            }
            // Try to move by y
            if translation.y > 0.0 {
                debug!("y > 0");
                let y_translation = apply_movement_in_one_direction(
                    &obstructions,
                    (Direction::South, translation.y),
                    &hitbox,
                    half_girth,
                );
                transform.translation.y = render_y_from_world_array_y(y_translation);
            } else if translation.y < 0.0 {
                debug!("y < 0");
                let y_translation = apply_movement_in_one_direction(
                    &obstructions,
                    (Direction::North, translation.y),
                    &hitbox,
                    half_girth,
                );
                transform.translation.y = render_y_from_world_array_y(y_translation);
            }
        } else {
            todo!("Non-Hitbox movement is not implemented.")
        }
    }
}
