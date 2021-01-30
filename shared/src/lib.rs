use glam::f32::Vec2;
use legion::Registry;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayerInputEvent {
    Move(Vec2),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NetworkMessage {
    PlayerInputEvent(PlayerInputEvent),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Player {
    pub id: u16,
}

pub fn get_registry() -> Registry<String> {
    let mut registry = Registry::<String>::default();
    registry.register::<Position>("position".to_string());
    registry.register::<Player>("player".to_string());

    registry
}
