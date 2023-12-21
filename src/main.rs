pub mod events;
mod systems;

pub mod enemy;
mod player;
pub mod score;
mod star;   

use events::*;
use systems::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_event::<GameOver>()
    .add_plugins(EnemyPlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(ScorePlugin)
    .add_plugins(StarPlugin)
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, exit_game)
    .add_systems(Update, handle_gameover)
    .run();
}