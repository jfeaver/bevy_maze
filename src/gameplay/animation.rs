//! Player sprite animation.

use bevy::prelude::*;
use std::time::Duration;

use crate::{
    AppSystems, PausableSystems,
    gameplay::{SpriteSheet, environment::Direction, movement::MovementController},
};

pub(super) fn plugin(app: &mut App) {
    // Animate and play sound effects based on controls.
    app.add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSystems::TickTimers),
            (update_animation_movement, update_animation_atlas)
                .chain()
                .run_if(resource_exists::<SpriteSheet>)
                .in_set(AppSystems::Update),
        )
            .in_set(PausableSystems),
    );
}

/// Update the sprite direction and animation state (idling/walking).
fn update_animation_movement(
    mut player_query: Query<(&MovementController, &mut PlayerAnimation, &mut Sprite)>,
) {
    for (controller, mut animation, sprite) in &mut player_query {
        let maybe_animation_state = if controller.intent == Vec2::ZERO {
            match &animation.state {
                PlayerAnimationState::Idling(direction) => {
                    Some(PlayerAnimationState::Idling(direction.clone()))
                }
                PlayerAnimationState::Walking(direction) => {
                    Some(PlayerAnimationState::Idling(direction.clone()))
                }
            }
        } else if controller.intent.y < 0.0 && controller.intent.x == 0.0 {
            Some(PlayerAnimationState::Walking(Direction::North))
        } else if controller.intent.y > 0.0 && controller.intent.x == 0.0 {
            Some(PlayerAnimationState::Walking(Direction::South))
        } else if controller.intent.x < 0.0 && controller.intent.y == 0.0 {
            Some(PlayerAnimationState::Walking(Direction::West))
        } else if controller.intent.x > 0.0 && controller.intent.y == 0.0 {
            Some(PlayerAnimationState::Walking(Direction::East))
        } else {
            None
        };
        if let Some(animation_state) = maybe_animation_state {
            animation.update_state(animation_state, sprite);
        }
    }
}

/// Update the animation timer.
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut PlayerAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(mut query: Query<(&PlayerAnimation, &mut Sprite)>) {
    for (animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

/// Component that tracks player's animation state.
/// It is tightly bound to the texture atlas we use.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    frame: usize,
    state: PlayerAnimationState,
}

#[derive(Reflect, PartialEq)]
pub enum PlayerAnimationState {
    Idling(Direction),
    Walking(Direction),
}

impl PlayerAnimation {
    /// The number of idle frames.
    const IDLE_FRAMES: usize = 2;
    /// The duration of each idle frame.
    const IDLE_INTERVAL: Duration = Duration::from_millis(600);
    /// The number of walking frames.
    const WALKING_FRAMES: usize = 4;
    /// The duration of each walking frame.
    const WALKING_INTERVAL: Duration = Duration::from_millis(200);

    fn idling(direction: Direction) -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Idling(direction),
        }
    }

    fn walking(direction: Direction) -> Self {
        Self {
            timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Walking(direction),
        }
    }

    pub fn new(direction: Direction) -> Self {
        Self::idling(direction)
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.is_finished() {
            return;
        }
        self.frame = (self.frame + 1)
            % match self.state {
                PlayerAnimationState::Idling(_) => Self::IDLE_FRAMES,
                PlayerAnimationState::Walking(_) => Self::WALKING_FRAMES,
            };
    }

    /// Update animation state if it changes.
    pub fn update_state(&mut self, state: PlayerAnimationState, mut sprite: Mut<Sprite>) {
        if self.state != state {
            match state {
                PlayerAnimationState::Idling(direction) => *self = Self::idling(direction),
                PlayerAnimationState::Walking(direction) => *self = Self::walking(direction),
            }
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                atlas.index = self.get_atlas_index();
            }
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.is_finished()
    }

    /// Return sprite index in the atlas.
    pub fn get_atlas_index(&self) -> usize {
        match &self.state {
            PlayerAnimationState::Idling(direction) => match direction {
                Direction::North => 120 + self.frame,
                Direction::South => 105 + self.frame,
                Direction::East => 135 + self.frame,
                Direction::West => 150 + self.frame,
            },
            PlayerAnimationState::Walking(direction) => match direction {
                Direction::North => 121 + self.frame,
                Direction::South => 106 + self.frame,
                Direction::East => 136 + self.frame,
                Direction::West => 151 + self.frame,
            },
        }
    }
}
