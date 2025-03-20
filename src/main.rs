mod camera;
mod food;
mod grid;
mod player;

use bevy::prelude::*;

use camera::CameraPlugin;
use food::FoodPlugin;
use grid::GridPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1920., 1080.).into(),
                title: "B3D".into(),
                visible: true,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((PlayerPlugin, CameraPlugin, GridPlugin, FoodPlugin))
        .run();
}
