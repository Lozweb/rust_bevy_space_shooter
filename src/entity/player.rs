use bevy::prelude::{Component, KeyCode, Query, Res, Time, Transform, With};
use bevy::input::{Input};
use bevy::sprite::TextureAtlasSprite;

#[derive(Component)]
pub struct Player {}

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query : Query<(&mut Transform, &mut TextureAtlasSprite), With<Player>>
) {

    let mut x_direction = 0.0;
    let mut y_direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        x_direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right){
        x_direction += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        y_direction += 1.0;
    }
    if  keyboard_input.pressed(KeyCode::Down) {
        y_direction -= 1.0;
    }

    for (mut transform, mut sprite) in &mut query {
        if y_direction > 0.0 { sprite.index = 1; }
        if y_direction < 0.0 { sprite.index = 3; }
        if y_direction == 0.0 { sprite.index = 0; }

        if transform.translation.x <= -610.0 { x_direction = 1.0; }
        if transform.translation.x >= 610.0 { x_direction = -1.0; }
        if transform.translation.y >= 340.0 { y_direction = -1.0; }
        if transform.translation.y <= -340.0 { y_direction = 1.0; }

        println!("x pos : {}", transform.translation.x);
        println!("y pos : {}", transform.translation.y);

        transform.translation.x += x_direction * 200.0 * time.delta_seconds();
        transform.translation.y += y_direction * 200.0 * time.delta_seconds();
    }
}