use bevy::{sprite::Sprite, prelude::{Bundle, Transform, Component, GlobalTransform, Handle, Image, Visibility}};

use super::movement::{Velocity, GravResist, GravBody};

#[derive(Bundle)]
pub struct PlayerBundle {
    // - Movement -
    pub global_transform: GlobalTransform,
    pub transform: Transform,
    pub velocity: Velocity,
    pub grav_body: GravBody,
    pub grav_resist: GravResist,
    // - Player -
    pub player: Player,
    pub health: Health,
    // - Sprite -
    pub sprite: Sprite,
    pub texture: Handle<Image>,
    pub visibility: Visibility, // User indication of whether an entity is visible
}

#[derive(Component)]
pub struct Health(u32);

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
    pub jump_strength: f32,
    pub jump_count: JumpCount,
}

#[derive(Component)]
pub enum PlayerState {
    Idle,
    Run, Jump, Fall
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Component)]
pub struct JumpCount {
    current: u8,
    max: u8,
}

impl Default for JumpCount {
    fn default() -> Self {
        Self::with(1)
    }
}

impl JumpCount {
    pub fn with(count: u8) -> Self {
        Self { current: count, max: count }
    }

    pub fn has_left(&self) -> bool {
        self.current != 0
    }

    pub fn take(&mut self) {
        self.current = self.current.saturating_sub(1);
    }

    pub fn give(&mut self) {
        self.current = self.current.saturating_add(1).min(self.max);
    }

    pub fn current(&self) -> u8 {
        self.current
    }
}