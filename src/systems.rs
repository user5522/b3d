use bevy::{
    prelude::*,
    window::{CursorGrabMode, WindowMode},
};
use bevy_egui::{EguiContexts, egui};

use crate::{components::PauseUI, resources::Sensitivity, states::GameState};

pub fn toggle_fullscreen(
    mut windows: Query<&mut Window>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F11) {
        if let Ok(mut window) = windows.get_single_mut() {
            window.mode = match window.mode {
                WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                _ => WindowMode::Windowed,
            }
        }
    }
}

pub fn toggle_cursor_lock(keys: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
    if keys.just_pressed(KeyCode::Escape) {
        if let Ok(mut window) = windows.get_single_mut() {
            match window.cursor_options.grab_mode {
                CursorGrabMode::Locked => {
                    window.cursor_options.grab_mode = CursorGrabMode::None;
                    window.cursor_options.visible = true;
                }
                _ => {
                    window.cursor_options.grab_mode = CursorGrabMode::Locked;
                    window.cursor_options.visible = false;
                }
            }
        }
    }
}

pub fn toggle_pause(
    mut next_state: ResMut<NextState<GameState>>,
    mut time: ResMut<Time<Virtual>>,
    state: Res<State<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::Running => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Running),
        }

        if time.is_paused() {
            time.unpause();
        } else {
            time.pause();
        }
    }
}

pub fn pause_ui(
    mut contexts: EguiContexts,
    mut commands: Commands,
    pause_ui_query: Query<Entity, With<PauseUI>>,
    mut sensitivity: ResMut<Sensitivity>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Settings")
        .anchor(egui::Align2::LEFT_CENTER, egui::Vec2::new(50., 0.))
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.set_width(250.);

            ui.add_space(10.);
            ui.label("Mouse Sensitivity");
            let slider = egui::Slider::new(&mut sensitivity.value, 0.001..=0.01).text("Value");
            ui.add(slider);
            ui.add_space(10.);
        });

    if !pause_ui_query.is_empty() {
        return;
    }

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            PauseUI,
            PickingBehavior::IGNORE,
        ))
        .with_children(|parent| {
            parent
                .spawn((Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(50.0),
                    left: Val::Px(50.0),
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Paused"),
                        TextFont {
                            font_size: 75.,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Label,
                    ));
                });
        });
}

pub fn despawn_pause_ui(mut commands: Commands, pause_ui_query: Query<Entity, With<PauseUI>>) {
    for entity in pause_ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
