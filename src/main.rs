mod camera;
mod food;
mod grid;
mod player;

mod components;
mod states;
mod systems;

use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

use camera::CameraPlugin;
use food::FoodPlugin;
use grid::GridPlugin;
use player::PlayerPlugin;

use states::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1920., 1080.).into(),
                title: "B3D".into(),
                mode: WindowMode::Windowed,
                visible: true,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            WorldInspectorPlugin::default(),
        ))
        .init_state::<GameState>()
        .add_plugins((PlayerPlugin, CameraPlugin, GridPlugin, FoodPlugin))
        .add_systems(
            Update,
            (toggle_fullscreen, toggle_cursor_lock, toggle_pause),
        )
        .add_systems(Update, pause_ui.run_if(in_state(GameState::Paused)))
        .add_systems(OnExit(GameState::Paused), despawn_pause_ui)
        .run();
}
