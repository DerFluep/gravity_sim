use crate::vec2::Vec2;
use crate::{
    ANGULAR_VELOCITY, ANGULAR_VELOCITY_FALLOFF, DISK_DENSITY_DISTRIBUTION, DISK_SIZE, MASS,
};
use ::rand::prelude::*;

use std::f64::consts::TAU;

pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f64,
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2, mass: f64) -> Self {
        Particle {
            position,
            velocity,
            mass,
        }
    }
}

pub fn create_particles(num: usize) -> Vec<Particle> {
    let mut rng = ::rand::rng();
    let mut particles = Vec::new();
    for _ in 0..num {
        let r = DISK_SIZE * rng.random::<f64>().powf(DISK_DENSITY_DISTRIBUTION);
        let theta = rng.random_range(0.0..TAU);
        let pos_x = r * theta.cos();
        let pos_y = r * theta.sin();
        let distance = (pos_x.powi(2) + pos_y.powi(2)).sqrt();
        let vel_x =
            -pos_y / (distance / DISK_SIZE).powf(ANGULAR_VELOCITY_FALLOFF) * ANGULAR_VELOCITY;
        let vel_y =
            pos_x / (distance / DISK_SIZE).powf(ANGULAR_VELOCITY_FALLOFF) * ANGULAR_VELOCITY;
        particles.push(Particle::new(
            Vec2::new(pos_x, pos_y),
            Vec2::new(vel_x, vel_y),
            MASS,
        ));
    }
    particles
}
