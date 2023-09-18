use bevy::prelude::{App, Plugin, Startup, Update};
use leafwing_input_manager::plugin::InputManagerPlugin;
use crate::systems::inputs::{Action, jump, spawn_player};

pub struct InputsPlugin;

impl Plugin for InputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(Startup, spawn_player)
            .add_systems(Update, jump);
    }
}