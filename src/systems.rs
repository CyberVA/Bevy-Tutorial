use bevy::{prelude::*, window::PrimaryWindow, app::AppExit, transform::commands};

use crate::{events::*, AppState, game::SimulationState};

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

pub fn transition_to_game_state (
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.get().clone() != AppState::Game {
            next_app_state.set(AppState::Game);
            println!("Entered Game State");
        }
    }
}

pub fn transition_to_main_menu_state (
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.get().clone() != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            next_sim_state.set(SimulationState::Paused);
            println!("Entered Main Menu State");
        }
    }
}

pub fn exit_game (
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        println!("Exiting game...");
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_gameover (
    mut game_over_event_reader: EventReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.read() {
        println!("Game Over! Your score is: {}", event.score.to_string());
        next_app_state.set(AppState::GameOver);
    }
}