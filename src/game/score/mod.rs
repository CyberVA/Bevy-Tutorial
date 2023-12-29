use bevy::prelude::*;

mod systems;
pub mod resources;

use resources::*;
use systems::*;
use super::SimulationState;
use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        // Startup Systems
        .add_systems(OnEnter(AppState::Game), insert_score)
        // Systems
        .add_systems(Update,
            (
                update_score,
            ).run_if(in_state(AppState::Game))
        )
        .add_systems(Update,
            (
                update_high_scores,
                high_scores_updated,
            )
        )
        // Exit Systems
        .add_systems(OnExit(AppState::Game), remove_score);
    }
}