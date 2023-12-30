pub mod enemy;
mod player;
pub mod score;
mod star;
mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use bevy::prelude::*;

use crate::events::GameOver;
use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<SimulationState>()
        // Events
        .add_event::<GameOver>()
        // Plugins
        .add_plugins((
            PlayerPlugin,
            EnemyPlugin,
            ScorePlugin,
            StarPlugin,
        ))
        // Startup Systems
        .add_systems(OnEnter(AppState::Game), pause_simulation)
        // Systems
        .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
        // Exit Systems
        .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}