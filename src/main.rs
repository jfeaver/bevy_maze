// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod asset_tracking;
mod audio;
#[cfg(feature = "dev")]
mod dev_tools;
mod gameplay;
mod menus;
mod screens;
mod theme;

use bevy::{asset::AssetMetaCheck, diagnostic::FrameCount, prelude::*, window::WindowResolution};

// Number of tiles (square) that fit on screen
const SCREEN_DIM: u8 = 11;
const SCREEN_PADDING: f32 = 0.5;
const PIXELS_PER_TILE: u8 = 16;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Maze".to_string(),
                        name: Some("Maze".into()),
                        // Tells Wasm to resize the window according to the available canvas
                        fit_canvas_to_parent: true,
                        // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        visible: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            audio::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            gameplay::plugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
        ));

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);
        // Configure the window just after loading
        app.add_systems(Update, show_and_config_primary_window);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            near: -100.0,
            far: 100.0,
            // viewport_origin: todo!(),
            scaling_mode: bevy::camera::ScalingMode::FixedVertical {
                viewport_height: (SCREEN_DIM as f32 + SCREEN_PADDING),
            },
            // scale: 0.8,
            // area: todo!(),
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn show_and_config_primary_window(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
    let screen_px = ((SCREEN_DIM as f32 + SCREEN_PADDING) * (PIXELS_PER_TILE as f32) * 9.0) as u32;
    if frames.0 == 1 {
        window.resolution = WindowResolution::new(screen_px, screen_px);
    }
    if frames.0 == 2 {
        window.position = WindowPosition::Centered(MonitorSelection::Current);
    }
    // The delay may be different for your app or system.
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.visible = true;
    }
}
