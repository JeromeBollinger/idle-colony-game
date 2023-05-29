use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::map::*;

pub const PLAYER_SPEED: f32 = 300.0;
pub const PLAYER_HEIGTH: f32 = 16.0;
pub const PLAYER_WIDTH: f32 = 16.0;

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 3.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(1.0, 1.0, 1.0),
            },
            texture: asset_server.load("player.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn collide_2d (rect1_x: f32,
                   rect1_width: f32,
                   rect1_y: f32,
                   rect1_heigth: f32,
                   rect2_x: f32,
                   rect2_width: f32,
                   rect2_y: f32,
                   rect2_heigth: f32,
) -> bool {
    if collide_1d(rect1_x, rect1_width, rect2_x, rect2_width) &&
        collide_1d(rect1_y, rect1_heigth, rect2_y, rect2_heigth){
            return true;
        }
    false
}

pub fn collide_1d (rect1_p: f32, rect1_length: f32,
                   rect2_p: f32, rect2_length: f32,) -> bool {
    if rect1_p + rect1_length <= rect2_p ||
        rect1_p >= rect2_p + rect2_length {
            return false;
        }
    true
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Camera>, Without<WallMapComponent>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>, Without<WallMapComponent>)>,
    map_query: Query<&Transform, (With<WallMapComponent>, Without<Player>, Without<Camera>)>,
    solid_walls: Query<&TilePos, With<Solid>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = player_query.get_single_mut() else {
        return;
    };
    let Ok(map) = map_query.get_single() else {
        return;
    };
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        let mut collision = false;
        for wall in solid_walls.iter(){
            if collide_2d(transform.translation.x - PLAYER_SPEED * time.delta_seconds(), PLAYER_WIDTH,
                          transform.translation.y, PLAYER_HEIGTH,
                          map.translation.x + wall.x as f32 * TILE_WIDTH as f32, TILE_WIDTH as f32,
                          map.translation.y + wall.y as f32 * TILE_HEIGTH as f32, TILE_HEIGTH as f32) {
                collision = true;
            }
        }
        if !collision {direction += Vec3::new(-1.0, 0.0, 0.0)};
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        let mut collision = false;
        for wall in solid_walls.iter(){
            if collide_2d(transform.translation.x + PLAYER_SPEED * time.delta_seconds(), PLAYER_WIDTH,
                          transform.translation.y, PLAYER_HEIGTH,
                          map.translation.x + wall.x as f32 * TILE_WIDTH as f32, TILE_WIDTH as f32,
                          map.translation.y + wall.y as f32 * TILE_HEIGTH as f32, TILE_HEIGTH as f32) {
                collision = true;
            }
        }
        if !collision {direction += Vec3::new(1.0, 0.0, 0.0)};
    }
    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        let mut collision = false;
        for wall in solid_walls.iter(){
            if collide_2d(transform.translation.x, PLAYER_WIDTH,
                          transform.translation.y + PLAYER_SPEED * time.delta_seconds(), PLAYER_HEIGTH,
                          map.translation.x + wall.x as f32 * TILE_WIDTH as f32, TILE_WIDTH as f32,
                          map.translation.y + wall.y as f32 * TILE_HEIGTH as f32, TILE_HEIGTH as f32) {
                collision = true;
            }
        }
        if !collision {direction += Vec3::new(0.0, 1.0, 0.0)};
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        let mut collision = false;
        for wall in solid_walls.iter(){
            if collide_2d(transform.translation.x, PLAYER_WIDTH,
                          transform.translation.y - PLAYER_SPEED * time.delta_seconds(), PLAYER_HEIGTH,
                          map.translation.x + wall.x as f32 * TILE_WIDTH as f32, TILE_WIDTH as f32,
                          map.translation.y + wall.y as f32 * TILE_HEIGTH as f32, TILE_HEIGTH as f32) {
                collision = true;
            }
        }
        if !collision {direction += Vec3::new(0.0, -1.0, 0.0)};
    }
    if direction.length() > 0.0 {
        direction = direction.normalize();
    }
    transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        camera_transform.translation.x = transform.translation.x;
        camera_transform.translation.y = transform.translation.y;
    }
}

pub fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Q) {
        exit.send(AppExit);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn collide_1d_test() {
        assert!(!collide_1d(0.0, 1.0, 2.0, 1.0));
        assert!(!collide_1d(-5.0, 1.0, -4.0, 90.0));
        assert!(!collide_1d(-5.0, 1.0, -7.0, 2.0));
        assert!(collide_1d(-5.0, 1.0, -7.0, 90.0));
    }

    #[test]
    fn collide_2d_test() {
        assert!(!collide_2d(0.0, 1.0, 0.0, 1.0,
                            2.0, 1.0, 2.0, 1.0));
        assert!(!collide_2d(0.0, 1.0, 0.0, 1.0,
                            0.0, 1.0, 2.0, 1.0));
        assert!(!collide_2d(0.0, 1.0, 0.0, 1.0,
                            2.0, 1.0, 0.0, 1.0));
        assert!(collide_2d(0.0, 1.0, 0.0, 1.0,
                            0.5, 1.0, 0.0, 1.0));
        assert!(!collide_2d(0.0, 1.0, 0.0, 1.0,
                           -2.0, 1.0, 0.0, 1.0));
        assert!(!collide_2d(0.0, 1.0, 0.0, 1.0,
                            0.0, 1.0, -2.0, 1.0));
        assert!(!collide_2d(75.0, 16.0, 0.0, 16.0,
                            -248.0, 16.0, -8.0, 16.0));
        assert!(!collide_2d(332.0, 16.0, -207.0, 16.0,
                            -56.0, 16.0, -200.0, 16.0));
    }
}
