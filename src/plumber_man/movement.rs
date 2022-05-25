use bevy::{prelude::{Component, Query, With, Res, Transform}, core::Time};
use glam::Vec2;


pub struct GlobalGravity {
    direction: Vec2,
    magnitude: f32,
}

impl Default for GlobalGravity {
    fn default() -> Self {
        Self {
            direction: Vec2::new(0.0, 1.0),
            magnitude: Self::MIN_MAGNITUDE,
        }
    }
}

impl GlobalGravity {
    const MIN_MAGNITUDE: f32 = 1.0;
    const MAX_MAGNITUDE: f32 = 3.0;

    pub fn set_magnitude(&mut self, mag: f32) {
        self.magnitude = mag.clamp(Self::MIN_MAGNITUDE, Self::MAX_MAGNITUDE);
    }
    
    pub fn set_up(&mut self) {
        self.direction = Vec2::new(0.0, 1.0);
    }

    pub fn set_down(&mut self) {
        self.direction = Vec2::new(0.0, -1.0);
    }

    pub fn vec(&self) -> Vec2 {
        self.magnitude * self.direction
    }

    pub fn vec_with_resist(&self, overcome: &GravResist) -> Vec2 {
        (self.magnitude - overcome.0).clamp(Self::MIN_MAGNITUDE, Self::MAX_MAGNITUDE) 
            * self.direction
    }
}

#[derive(Component, Default)]
pub struct GravBody;

#[derive(Component)]
pub struct GravResist(pub f32);

impl Default for GravResist {
    fn default() -> Self {
        Self::none()
    }
}

impl GravResist {
    fn none() -> Self {
        Self(0.0)
    }
}

#[derive(Component)]
pub struct Velocity{
    pub vector: Vec2,
}


pub fn apply_gravity(
    time: Res<Time>,
    gravity: Res<GlobalGravity>,
    mut grav_bodies: Query<(&mut Velocity, Option<&GravResist>), With<GravBody>>,
) {
    let no_grav_resist = GravResist::none();
    for (mut velocity, grav_res) in grav_bodies.iter_mut() {
        let grav_res = grav_res.unwrap_or(&no_grav_resist);
        let grav_vec = gravity.vec_with_resist(&grav_res);
        velocity.vector += grav_vec * time.delta_seconds();
    }
}

pub fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>
) {
    for (v, mut t) in query.iter_mut() {
        t.translation += v.vector.extend(0.0) * time.delta_seconds();
    }
}