use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        // Systems
        .add_systems(Startup, main_menu);
    }
}

pub fn main_menu() {
    println!("You are in the main menu!");
}