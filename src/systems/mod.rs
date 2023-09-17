use bevy::math::*;
use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

// 添加相机
pub fn init_window(mut commands: Commands) -> () {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}

pub fn draw_grid(mut commands: Commands) {
    let (width_half, height_half) = MAP_HALF_SIZE;
    let (mut x, mut y) = (BLOCK_RADIUS, BLOCK_RADIUS);

    while x + BLOCK_SIZE <= width_half {
        x += BLOCK_SIZE;
    }
    while y + BLOCK_SIZE <= height_half {
        y += BLOCK_SIZE;
    }
    x -= BLOCK_SIZE;
    y -= BLOCK_SIZE;

    let x_copy = x.clone();
    while y - BLOCK_SIZE >= -height_half {
        while x - BLOCK_SIZE >= -width_half {
            spawn_sprite(&mut commands, x, y);
            x -= BLOCK_SIZE;
        }
        y -= BLOCK_SIZE;
        x = x_copy.clone();
    }
}

pub fn spawn_sprite(commands: &mut Commands, x: f32, y: f32) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(vec2(BLOCK_SIZE, BLOCK_SIZE)),
                ..default()
            },
            transform: Transform {
                translation: vec3(x, y, 0.),
                ..default()
            },
            ..default()
        })
        .insert(Outer);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::GRAY,
                custom_size: Some(vec2(CACHE_SIZE, CACHE_SIZE)),
                ..default()
            },
            transform: Transform {
                translation: vec3(x, y, 1.),
                ..default()
            },
            ..default()
        })
        .insert(Cache);
}


pub fn move_camera(buttons: Res<Input<KeyCode>>, mut camera: Query<&mut Transform, With<MainCamera>>) {
    let (x, y) = (camera.single().translation.x, camera.single().translation.y);
    let mut camera_single = camera.single_mut();
    if buttons.pressed(KeyCode::Left) {
        camera_single.translation.x = x - MOVE_SPEED;
    }
    if buttons.pressed(KeyCode::Right) {
        camera_single.translation.x = x + MOVE_SPEED;
    }
    if buttons.pressed(KeyCode::Up) {
        camera_single.translation.y = y + MOVE_SPEED;
    }
    if buttons.pressed(KeyCode::Down) {
        camera_single.translation.y = y - MOVE_SPEED;
    }
    // 边界条件
    if camera_single.translation.x > MAP_HALF_SIZE.0 - BLOCK_SIZE - 12. {
        camera_single.translation.x = MAP_HALF_SIZE.0 - BLOCK_SIZE - 12.;
    }
    if camera_single.translation.x < -MAP_HALF_SIZE.0 + BLOCK_SIZE + 12. {
        camera_single.translation.x = -MAP_HALF_SIZE.0 + BLOCK_SIZE + 12.;
    }
    if camera_single.translation.y > MAP_HALF_SIZE.1 - BLOCK_SIZE - 12. {
        camera_single.translation.y = MAP_HALF_SIZE.1 - BLOCK_SIZE - 12.;
    }
    if camera_single.translation.y < -MAP_HALF_SIZE.1 + BLOCK_SIZE + 12. {
        camera_single.translation.y = -MAP_HALF_SIZE.1 + BLOCK_SIZE + 12.;
    }
}

pub fn draw_people(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("gabe-idle-run.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform::from_xyz(0., 0., 3.),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .insert(People);
}

pub fn move_people(
    buttons: Res<Input<KeyCode>>,
    buttons_mouse: Res<Input<MouseButton>>,
    mut people: Query<&mut Transform, (With<People>, Without<Cache>)>,
    mut cache_block: Query<(&mut Sprite, &Transform), With<Cache>>,
    time: Res<Time>,
    animate_info: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
    windows: Query<&mut Window>,
    camera: Query<(&mut Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (x, y) = (people.single().translation.x, people.single().translation.y);
    let mut people_single = people.single_mut();
    if buttons.pressed(KeyCode::Left) {
        people_single.translation.x = x - MOVE_SPEED;
    }
    if buttons.pressed(KeyCode::Right) {
        people_single.translation.x = x + MOVE_SPEED;
        animate_sprite(time, animate_info);
    }
    if buttons.pressed(KeyCode::Up) {
        people_single.translation.y = y + MOVE_SPEED;
    }
    if buttons.pressed(KeyCode::Down) {
        people_single.translation.y = y - MOVE_SPEED;
    }
    // 边界条件
    if people_single.translation.x > MAP_HALF_SIZE.0 - BLOCK_SIZE - 12. {
        people_single.translation.x = MAP_HALF_SIZE.0 - BLOCK_SIZE - 12.;
    }
    if people_single.translation.x < -MAP_HALF_SIZE.0 + BLOCK_SIZE + 12. {
        people_single.translation.x = -MAP_HALF_SIZE.0 + BLOCK_SIZE + 12.;
    }
    if people_single.translation.y > MAP_HALF_SIZE.1 - BLOCK_SIZE - 12. {
        people_single.translation.y = MAP_HALF_SIZE.1 - BLOCK_SIZE - 12.;
    }
    if people_single.translation.y < -MAP_HALF_SIZE.1 + BLOCK_SIZE + 16. {
        people_single.translation.y = -MAP_HALF_SIZE.1 + BLOCK_SIZE + 16.;
    }

    if buttons_mouse.pressed(MouseButton::Left) {
        let (camera_single, camera_transform) = camera.single();
        let window = windows.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera_single.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let (cursor_x, cursor_y) = (world_position.x, world_position.y);
            for (mut block, block_transform) in cache_block.iter_mut() {
                let (block_x, block_y) =
                    (block_transform.translation.x, block_transform.translation.y);
                let (block_width_half, block_height_half) = (
                    block.custom_size.unwrap().x / 2.,
                    block.custom_size.unwrap().y / 2.,
                );
                let is_this_block = cursor_x >= block_x - block_width_half - BLOCK_BOARD_SIZE
                    && cursor_x <= block_x + block_width_half + BLOCK_BOARD_SIZE
                    && cursor_y >= block_y - block_height_half - BLOCK_BOARD_SIZE
                    && cursor_y <= block_y + block_height_half + BLOCK_BOARD_SIZE;
                if is_this_block {
                    block.color = Color::PURPLE;
                }
            }
        }
    }
    let (people_x, people_y) = (people.single().translation.x, people.single().translation.y);
    for (mut block, block_transform) in cache_block.iter_mut() {
        let (block_x, block_y) =
            (block_transform.translation.x, block_transform.translation.y);
        let (block_width_half, block_height_half) = (
            block.custom_size.unwrap().x / 2.,
            block.custom_size.unwrap().y / 2.,
        );
        let is_this_block = people_x >= block_x - block_width_half - BLOCK_BOARD_SIZE - 12.
            && people_x <= block_x + block_width_half + BLOCK_BOARD_SIZE + 12.
            && people_y >= block_y - block_height_half - BLOCK_BOARD_SIZE - 12.
            && people_y <= block_y + block_height_half + BLOCK_BOARD_SIZE + 12.;
        if is_this_block && block.color == Color::PURPLE {
            block.color = Color::BLUE;
        }
    }
}

pub fn move_people_stop(
    buttons: Res<Input<KeyCode>>,
    animate_info: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    if !buttons.pressed(KeyCode::Right) {
        animate_sprite_stop(animate_info);
    }
}

// 移动动画
pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

// 停止移动动画
pub fn animate_sprite_stop(
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, _, mut sprite) in &mut query {
        sprite.index = indices.first - 1;
    }
}