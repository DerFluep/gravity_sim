#![allow(dead_code)]

mod particle;
mod quadtree;
mod square;
mod vec2;

use crate::particle::create_particles;
use crate::quadtree::Quadtree;
use macroquad::prelude::*;

use core::f32;

const PARTICLE_COUNT: usize = 100000;
const DISK_SIZE: f64 = 512.0;
const G: f64 = 0.005;
const ANGULAR_VELOCITY: f64 = 0.002;
const ANGULAR_VELOCITY_FALLOFF: f64 = 0.5;
const DISK_DENSITY_DISTRIBUTION: f64 = 0.8;
const MIN_MASS: f64 = 5.0;
const MAX_MASS: f64 = 12.0;
const MASS: f64 = 1.0;

//_______________________________________________________________

/*
struct Particles {
    num: usize,
    x: Vec<f32>,
    y: Vec<f32>,
    vel_x: Vec<f32>,
    vel_y: Vec<f32>,
    m: Vec<f32>,
}

impl Particles {
    pub fn new(num: usize) -> Self {
        let mut rng = ::rand::rng();
        let mut temp_x = Vec::new();
        let mut temp_y = Vec::new();
        let mut temp_vel_x: Vec<f32> = Vec::new();
        let mut temp_vel_y: Vec<f32> = Vec::new();
        let mut temp_m: Vec<f32> = Vec::new();
        for n in 0..num {
            let r = DISK_SIZE * rng.random::<f32>().powf(DISK_DENSITY_DISTRIBUTION);
            let theta = rng.random_range(0.0..TAU);
            temp_x.push(r * theta.cos());
            temp_y.push(r * theta.sin());
            let distance = (temp_x[n].powi(2) + temp_y[n].powi(2)).sqrt();
            temp_vel_x.push(
                -temp_y[n] / (distance / DISK_SIZE).powf(ANGULAR_VELOCITY_FALLOFF)
                    * ANGULAR_VELOCITY,
            );
            temp_vel_y.push(
                temp_x[n] / (distance / DISK_SIZE).powf(ANGULAR_VELOCITY_FALLOFF)
                    * ANGULAR_VELOCITY,
            );
            temp_m.push(rng.random_range(MIN_MASS..MAX_MASS));
        }
        Particles {
            num,
            x: temp_x,
            y: temp_y,
            vel_x: temp_vel_x,
            vel_y: temp_vel_y,
            m: temp_m,
        }
    }

    pub fn restart(&mut self) {
        let mut rng = ::rand::rng();
        let mut temp_x = Vec::new();
        let mut temp_y = Vec::new();
        let mut temp_vel_x: Vec<f32> = Vec::new();
        let mut temp_vel_y: Vec<f32> = Vec::new();
        let mut temp_m: Vec<f32> = Vec::new();
        for n in 0..self.num {
            let r = DISK_SIZE * rng.random::<f32>().powf(DISK_DENSITY_DISTRIBUTION);
            let theta = rng.random_range(0.0..TAU);
            temp_x.push(r * theta.cos());
            temp_y.push(r * theta.sin());
            let distance = (temp_x[n].powi(2) + temp_y[n].powi(2)).sqrt();
            temp_vel_x.push(
                -temp_y[n]
                    * (distance / DISK_SIZE).powf(ANGULAR_VELOCITY_FALLOFF)
                    * ANGULAR_VELOCITY,
            );
            temp_vel_y.push(
                temp_x[n]
                    * (distance / DISK_SIZE).powf(ANGULAR_VELOCITY_FALLOFF)
                    * ANGULAR_VELOCITY,
            );
            temp_m.push(rng.random_range(MIN_MASS..MAX_MASS));
        }
        self.x = temp_x;
        self.y = temp_y;
        self.vel_x = temp_vel_x;
        self.vel_y = temp_vel_y;
        self.m = temp_m;
    }

    pub fn update(&mut self) {
        /*
                // collision detection and merging
                let mut i = 0;
                while i < self.x.len() {
                    let mut merged = false;
                    let mut j = i + 1;
                    while j < self.x.len() {
                        let dir_x = self.x[i] - self.x[j];
                        let dir_y = self.y[i] - self.y[j];
                        let distance = (dir_x.powi(2) + dir_y.powi(2)).sqrt();
                        let radius_i = ((self.m[i] * 3.0) / (4.0 * PI)).cbrt();
                        let radius_j = ((self.m[j] * 3.0) / (4.0 * PI)).cbrt();
                        if distance < radius_i + radius_j {
                            if self.m[i] < self.m[j] {
                                self.x[i] = self.x[j];
                                self.y[i] = self.y[j];
                            };
                            let mid_vel_x = (self.m[i] * self.vel_x[i] + self.m[j] * self.vel_x[j])
                                / (self.m[i] + self.m[j]);
                            let mid_vel_y = (self.m[i] * self.vel_y[i] + self.m[j] * self.vel_y[j])
                                / (self.m[i] + self.m[j]);
                            let new_mass = self.m[i] + self.m[j];

                            self.x.remove(j);
                            self.y.remove(j);
                            self.vel_x.remove(j);
                            self.vel_y.remove(j);
                            self.m.remove(j);

                            self.vel_x[i] = mid_vel_x;
                            self.vel_y[i] = mid_vel_y;
                            self.m[i] = new_mass;

                            merged = true;
                            break;
                        } else {
                            j += 1;
                        }
                    }
                    if !merged {
                        i += 1;
                    }
                }
        */

        // Position updating
        let len = self.x.len();
        let results: Vec<(f32, f32, f32, f32)> = (0..len)
            .into_par_iter()
            .map(|current| {
                let mut vel_x = self.vel_x[current];
                let mut vel_y = self.vel_y[current];
                for n in 0..len {
                    if n == current {
                        continue;
                    }
                    let mut dir_x = self.x[n] - self.x[current];
                    let mut dir_y = self.y[n] - self.y[current];
                    let mut distance = (dir_x.powi(2) + dir_y.powi(2)).sqrt();
                    if distance < 5.0 {
                        distance = 5.0;
                    }
                    dir_x /= distance;
                    dir_y /= distance;
                    let force = G * (self.m[current] * self.m[n]) / distance.powi(2);
                    let acceleration = force / self.m[current];
                    vel_x += dir_x * acceleration;
                    vel_y += dir_y * acceleration;
                }

                let new_x = self.x[current] + vel_x;
                let new_y = self.y[current] + vel_y;

                (new_x, new_y, vel_x, vel_y)
            })
            .collect();

        for (i, (nx, ny, nvx, nvy)) in results.into_iter().enumerate() {
            self.x[i] = nx;
            self.y[i] = ny;
            self.vel_x[i] = nvx;
            self.vel_y[i] = nvy;
        }
    }
}
*/

#[macroquad::main("Gravity Particle Sim")]
async fn main() {
    let mut particles = create_particles(PARTICLE_COUNT);
    let start = std::time::Instant::now();
    let tree = Quadtree::create(&particles);
    let end = std::time::Instant::now();
    println!("Tree creation took: {:?}", end - start);

    let mut view_off_x = screen_width();
    let mut view_off_y = screen_height();

    let mut run = false;

    'running: loop {
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            break 'running;
        }
        if is_key_pressed(KeyCode::Space) {
            run = !run;
        }
        if is_key_pressed(KeyCode::Enter) {
            view_off_x = 0.0;
            view_off_y = 0.0;
        }
        if is_key_down(KeyCode::Up) {
            view_off_y += 5.0;
        }
        if is_key_down(KeyCode::Down) {
            view_off_y -= 5.0;
        }
        if is_key_down(KeyCode::Left) {
            view_off_x += 5.0;
        }
        if is_key_down(KeyCode::Right) {
            view_off_x -= 5.0;
        }

        clear_background(BLACK);

        // TODO: implement toggle for box drawing
        tree.draw(view_off_x, view_off_y);

        // TODO: render a pixel texture instead of drawing separate circles
        particles.iter().for_each(|particle| {
            draw_circle(
                particle.position.x as f32 + view_off_x,
                particle.position.y as f32 + view_off_y,
                1.0,
                WHITE,
            );
        });

        draw_fps();
        next_frame().await
    }
}
