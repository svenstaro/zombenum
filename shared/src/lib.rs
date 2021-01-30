use serde::{Serialize, Deserialize};
use glam::f32::Vec2;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayerInputEvent {
    Move(Vec2)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NetworkMessage {
    PlayerInputEvent(PlayerInputEvent)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    id: u16,
}
