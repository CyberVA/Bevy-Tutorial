use bevy::prelude::*;

pub mod components;
mod systems;
pub mod resources;

use resources::*;
use systems::*;

use super::SimulationState;
use crate::AppState;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<EnemySpawnTimer>()
        // Startup Systems
        .add_systems(OnEnter(AppState::Game), spawn_enemies)
        // Systems
        .add_systems(Update,
            (
                enemy_movement,
                update_enemy_direction,
                confine_enemy_movement,
                tick_enemy_spawn_timer,
                spawn_enemies_over_time,
            ).run_if(in_state(AppState::Game))
             .run_if(in_state(SimulationState::Running))
        )
        // Exit Systems
        .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}