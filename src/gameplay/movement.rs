use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    gameplay::{
        environment::WorldMap,
        utils::{Hitbox, world_array_position_from_render_position},
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
    /// The direction the character wants to move in.
    pub intent: Vec2,

    /// Maximum speed in world units per second.
    /// 1 world unit = 1 pixel when using the default 2D camera and no physics engine.
    pub max_speed: f32,

    pub hitbox: Option<Hitbox>,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            // three tiles per second is a nice default, but we can still vary this per character.
            max_speed: 3.0,
            hitbox: None,
        }
    }
}

fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &mut Transform)>,
    world_map: Res<WorldMap>,
) {
    // FIXME:
    // The hitbox extents aren't checked to see if they are constraining. Only one corner.
    // Both corners of the hitbox that are on the same tangent line to the direction of travel should be checked for collisions
    for (controller, mut transform) in &mut movement_query {
        if controller.intent == Vec2::ZERO {
            return;
        }
        let velocity = controller.max_speed * controller.intent;
        let translation = velocity.extend(0.0) * time.delta_secs(); // world array space
        // let mut collision_vectors: Vec<Vec3> = Vec::new();
        // if let Some(hitbox) = controller.hitbox {
        //     let mut cv1 = translation.clone();
        //     let mut cv2 = translation.clone();
        //     if translation.x > 0.0 {
        //         cv1.x += hitbox.rect.width();
        //         cv2.x += hitbox.rect.width();
        //         cv2.y += hitbox.rect.height();
        //     } else if translation.x < 0.0 {
        //         cv2.y += hitbox.rect.height();
        //     }
        //     if translation.y > 0.0 {
        //         cv1.y += hitbox.rect.height();
        //         cv2.y += hitbox.rect.height();
        //         cv2.x += hitbox.rect.width();
        //     } else if translation.y < 0.0 {
        //         cv2.x += hitbox.rect.width();
        //     }
        //     collision_vectors.push(cv1);
        //     collision_vectors.push(cv2);
        // } else {
        //     collision_vectors.push(translation.clone());
        // }
        let collision_vector = {
            if let Some(hitbox) = controller.hitbox {
                let mut t = translation.clone();
                if t.x > 0.0 {
                    t.x += hitbox.rect.width();
                }
                if t.y > 0.0 {
                    t.y += hitbox.rect.height();
                }
                t
            } else {
                translation
            }
        };
        let position = world_array_position_from_render_position(
            transform.translation.x,
            transform.translation.y,
        ); // world array space
        debug!("[move plan] {:?}", position + translation.truncate());
        if translation.x != 0.0 {
            // if collision_vectors.iter().any(|cv: &Vec3| {
            //     world_map
            //         .at(position + Vec2::new(cv.x, 0.0))
            //         .map(|tile| tile.is_obstruction())
            //         .unwrap_or(false)
            // }) {
            //     transform.translation.x += translation.x; // world position space
            //     //     debug!("X MOVE {:?}", x_collision);
            //     // } else {
            //     //     debug!(
            //     //         "x_move obstruction ({:?} -> {:?}) with collider {:?}",
            //     //         position, x_collision, tile
            //     //     );
            // }
            let x_collision = position + Vec2::new(collision_vector.x, 0.0);
            if let Some(tile) = world_map.at(x_collision) {
                if !tile.is_obstruction() {
                    transform.translation.x += translation.x; // world position space
                    debug!("X MOVE {:?}", x_collision);
                } else {
                    debug!(
                        "x_move obstruction ({:?} -> {:?}) with collider {:?}",
                        position, x_collision, tile
                    );
                }
            }
        }
        if translation.y != 0.0 {
            // if collision_vectors.iter().any(|cv: &Vec3| {
            //     world_map
            //         .at(position + Vec2::new(0.0, cv.y))
            //         .map(|tile| tile.is_obstruction())
            //         .unwrap_or(false)
            // }) {
            //     transform.translation.x += translation.x; // world position space
            //     //     debug!("X MOVE {:?}", x_collision);
            //     // } else {
            //     //     debug!(
            //     //         "x_move obstruction ({:?} -> {:?}) with collider {:?}",
            //     //         position, x_collision, tile
            //     //     );
            // }
            let y_collision = position + Vec2::new(0.0, collision_vector.y);
            if let Some(tile) = world_map.at(y_collision) {
                if !tile.is_obstruction() {
                    transform.translation.y -= translation.y; // subtract to translate to world position space
                    debug!("Y MOVE {:?}", y_collision);
                } else {
                    debug!(
                        "y_move obstruction ({:?} -> {:?}) with collider {:?}",
                        position, y_collision, tile
                    );
                }
            }
        }
    }
}
