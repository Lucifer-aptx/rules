use bevy::prelude::{Camera2dBundle, Commands, Component};
// 主相机
#[derive(Component)]
pub struct MainCamera;
pub fn init(mut commands: Commands){
    commands.spawn(Camera2dBundle::default())
        .insert(MainCamera);
}