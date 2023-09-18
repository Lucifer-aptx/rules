use bevy::prelude::{Camera2dBundle, Commands};
use crate::components::MainCamera;

pub fn init(mut commands: Commands){
    commands.spawn(Camera2dBundle::default())
        .insert(MainCamera);
}