use bevy::prelude::{Commands, Component, KeyCode, Query, With};
use bevy::reflect::TypePath;
use leafwing_input_manager::{Actionlike, InputManagerBundle};
use leafwing_input_manager::prelude::{ActionState, InputMap};

#[derive(Actionlike, TypePath, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    Jump,
}

#[derive(Component)]
pub struct Player;

pub fn jump(query: Query<&ActionState<Action>, With<Player>>) {
    let action_state = query.single();
    // Each action has a button-like state of its own that you can check
    if action_state.just_pressed(Action::Jump) {
        println!("I'm jumping!");
    }
}

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn(InputManagerBundle::<Action> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map: InputMap::new([(KeyCode::Space, Action::Jump)]),
        })
        .insert(Player);
}