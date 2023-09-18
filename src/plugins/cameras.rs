use bevy::app::{App, Startup};
use bevy::prelude::Plugin;

pub struct CamerasPlugin;

impl Plugin for CamerasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, crate::systems::cameras::init);
    }
}