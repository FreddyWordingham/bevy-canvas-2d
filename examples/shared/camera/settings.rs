use bevy::prelude::*;

pub const CLEAR_COLOUR: Color = Color::BLACK;

pub const ZOOM_SPEED: f32 = 1.0;
pub const PAN_SPEED: f32 = 1.0e3;
pub const SPEED_MULTIPLIER: f32 = 5.0;

pub const ZOOM_IN: KeyCode = KeyCode::KeyQ;
pub const ZOOM_OUT: KeyCode = KeyCode::KeyE;
pub const PAN_UP: KeyCode = KeyCode::KeyW;
pub const PAN_DOWN: KeyCode = KeyCode::KeyS;
pub const PAN_LEFT: KeyCode = KeyCode::KeyA;
pub const PAN_RIGHT: KeyCode = KeyCode::KeyD;
