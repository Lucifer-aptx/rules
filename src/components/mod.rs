use bevy::prelude::{Component, Deref, DerefMut, Timer};

#[derive(Component)]
pub struct Cache;

#[derive(Component)]
pub struct Outer;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct People;

#[derive(Component)]
pub struct AnimationIndices {
    pub(crate) first: usize,
    pub(crate) last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
