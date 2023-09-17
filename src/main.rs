use bevy::prelude::*;
use crate::systems::*;


pub mod components;
pub mod constants;
pub mod systems;
pub mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (init_window, draw_grid, draw_people))
        .add_systems(
            Update,
            (move_camera, move_people, move_people_stop),
        )
        .run();
}