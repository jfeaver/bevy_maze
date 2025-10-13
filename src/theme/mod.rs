//! Reusable UI widgets & theming.

// Unused utilities may trigger this lints undesirably.
#![allow(dead_code)]

pub mod interaction;
pub mod palette;
pub mod widget;

use bevy::app::App;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}
