use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

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
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_movement.in_set(MovementSystemSet))
        .add_systems(Update, confine_player_movement.in_set(ConfinementSystemSet))
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update, player_hit_star);
    }
}