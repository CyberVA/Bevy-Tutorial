pub mod events;
mod game;
mod main_menu;
mod systems;

use game::GamePlugin;
use main_menu::MainMenuPlugin;

use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
    // Bevy plugins
    .add_plugins(DefaultPlugins)
    .add_state::<AppState>()
    // My Plugins
    .add_plugins(MainMenuPlugin)
    .add_plugins(GamePlugin)
    // Startup Systems
    .add_systems(Startup, spawn_camera)
    // Systems
    .add_systems(Update, transition_to_game_state)
    .add_systems(Update, transition_to_main_menu_state)
    .add_systems(Update, exit_game)
    .add_systems(Update, handle_gameover)
    .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}