use bevy::prelude::*;

use super::settings::{
    CLEAR_COLOUR, PAN_DOWN, PAN_LEFT, PAN_RIGHT, PAN_SPEED, PAN_UP, SPEED_MULTIPLIER, ZOOM_IN, ZOOM_OUT, ZOOM_SPEED,
};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(CLEAR_COLOUR),
            ..default()
        },
    ));
}

pub fn zoom_camera(time: Res<Time>, keys: Res<ButtonInput<KeyCode>>, mut camera: Single<&mut Projection, With<Camera>>) {
    if let Projection::Orthographic(ref mut ortho) = **camera {
        let mut zoom = 0.0;
        if keys.pressed(ZOOM_IN) {
            zoom += 1.0;
        }
        if keys.pressed(ZOOM_OUT) {
            zoom -= 1.0;
        }
        if zoom == 0.0 {
            return;
        }

        let multiplier = if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
            SPEED_MULTIPLIER
        } else {
            1.0
        };

        let factor = 1.0 + (zoom * ZOOM_SPEED * multiplier * time.delta_secs());
        ortho.scale = ortho.scale * factor;
    }
}

pub fn pan_camera(time: Res<Time>, keys: Res<ButtonInput<KeyCode>>, mut camera: Single<&mut Transform, With<Camera>>) {
    let mut direction = Vec2::ZERO;
    if keys.pressed(PAN_UP) {
        direction.y += 1.0;
    }
    if keys.pressed(PAN_DOWN) {
        direction.y -= 1.0;
    }
    if keys.pressed(PAN_LEFT) {
        direction.x -= 1.0;
    }
    if keys.pressed(PAN_RIGHT) {
        direction.x += 1.0;
    }
    if direction == Vec2::ZERO {
        return;
    }

    let multiplier = if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
        SPEED_MULTIPLIER
    } else {
        1.0
    };

    let movement = PAN_SPEED * direction.normalize() * multiplier * time.delta_secs();
    camera.translation.x += movement.x;
    camera.translation.y += movement.y;
}
