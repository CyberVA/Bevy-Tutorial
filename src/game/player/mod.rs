use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;
use super::SimulationState;
use crate::AppState;

pub const PLAYER_SIZE: f32 = 64.0; // Player sprite size
pub const PLAYER_SPEED: f32 = 500.0; // Player speed

// System Sets
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .configure_sets(Update, MovementSystemSet.before(ConfinementSystemSet))
        // Startup Systems
        .add_systems(OnEnter(AppState::Game), spawn_player)
        // Systems
        .add_systems(Update,
            (
                player_movement.in_set(MovementSystemSet),
                confine_player_movement.in_set(ConfinementSystemSet),
                enemy_hit_player,
                player_hit_star,
            ).run_if(in_state(AppState::Game))
             .run_if(in_state(SimulationState::Running))
        )
        // Exit Systems
        .add_systems(OnExit(AppState::Game), despawn_player);
    }
}