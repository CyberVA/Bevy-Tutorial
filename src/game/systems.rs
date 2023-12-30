use bevy::prelude::*;

use crate::game::SimulationState;

pub fn pause_simulation (
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    next_sim_state.set(SimulationState::Paused);
}

pub fn resume_simulation (
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    next_sim_state.set(SimulationState::Running)
}

pub fn toggle_simulation (
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: ResMut<State<SimulationState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::P) {
        match simulation_state.get() {
            SimulationState::Running => {
                next_sim_state.set(SimulationState::Paused);
                println!("Game Paused");
            }
            SimulationState::Paused => {
                next_sim_state.set(SimulationState::Running);
                println!("Game Resumed");
            }
        }
    }
}