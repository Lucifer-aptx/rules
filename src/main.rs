use bevy::DefaultPlugins;
use bevy::prelude::App;
use crate::plugins::cameras::CamerasPlugin;
use crate::plugins::inputs::InputsPlugin;
use crate::plugins::physics::PhysicsPlugin;

pub mod constants;
pub mod systems;
pub mod plugins;
pub mod resources;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .add_plugins(CamerasPlugin)
        .add_plugins(InputsPlugin)
        .run();
}