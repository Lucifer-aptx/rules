use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use rand::prelude::*;

const BLOCK_SIZE: f32 = 15.;
const BLOCK_BOARD_SIZE: f32 = 0.5;
static CACHE_SIZE: f32 = BLOCK_SIZE - BLOCK_BOARD_SIZE;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (init_window, draw_grid))
        .add_systems(Update, click_with_color)
        .run();
}

/// 添加相机
fn init_window(mut commands: Commands) -> () {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}

fn draw_grid(mut commands: Commands, window: Query<&mut Window>) {
    let (width, height) = (
        window.single().resolution.width(),
        window.single().resolution.height(),
    );
    let (mut x, mut y) = (BLOCK_SIZE / 2., BLOCK_SIZE / 2.);

    while x + BLOCK_SIZE <= width / 2. {
        x += BLOCK_SIZE;
    }
    while y + BLOCK_SIZE <= height / 2. {
        y += BLOCK_SIZE;
    }
    x -= BLOCK_SIZE;
    y -= BLOCK_SIZE;

    let x_copy = x.clone();
    while y - BLOCK_SIZE >= -height / 2. {
        while x - BLOCK_SIZE >= -width / 2. {
            spawn_sprite(&mut commands, x, y);
            x -= BLOCK_SIZE;
        }
        y -= BLOCK_SIZE;
        x = x_copy.clone();
    }
}

fn spawn_sprite(commands: &mut Commands, x: f32, y: f32) {
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

fn click_with_color(
    mut cache_block: Query<(&mut Sprite, &Transform), With<Cache>>,
    buttons: Res<Input<MouseButton>>,
    windows: Query<&mut Window>,
    camera: Query<(&mut Camera, &GlobalTransform), With<MainCamera>>,
) {
    if buttons.pressed((MouseButton::Left)) {
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
                let (block_width, block_height) =
                    (block.custom_size.unwrap().x, block.custom_size.unwrap().y);
                if cursor_x >= block_x - block_width / 2. - BLOCK_BOARD_SIZE
                    && cursor_x <= block_x + block_width / 2. + BLOCK_BOARD_SIZE
                    && cursor_y >= block_y - block_height / 2. - BLOCK_BOARD_SIZE
                    && cursor_y <= block_y + block_height / 2. + BLOCK_BOARD_SIZE
                {
                    block.color = Color::PURPLE;
                }
            }
        }
    }
}
#[derive(Component)]
struct Cache;

#[derive(Component)]
struct Outer;

#[derive(Component)]
struct MainCamera;


