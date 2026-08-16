use std::time::Duration;

use rand::prelude::*;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::{Point, Rect};

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

struct Particles {
    x: Vec<f32>,
    y: Vec<f32>,
    vel_x: Vec<f32>,
    vel_y: Vec<f32>,
    m: Vec<f32>,
}

impl Particles {
    pub fn new(num: usize) -> Self {
        let mut rng = rand::rng();
        let mut temp_x = Vec::new();
        let mut temp_y = Vec::new();
        let mut temp_vel_x: Vec<f32> = Vec::new();
        let mut temp_vel_y: Vec<f32> = Vec::new();
        for _ in 0..num {
            temp_x.push(rng.random_range(0.0..WIDTH as f32));
            temp_y.push(rng.random_range(0.0..HEIGHT as f32));
            temp_vel_x.push(rng.random_range(-0.5..0.5));
            temp_vel_y.push(rng.random_range(-0.5..0.5));
        }
        Particles {
            x: temp_x,
            y: temp_y,
            vel_x: temp_vel_x,
            vel_y: temp_vel_y,
            m: vec![0.05; num],
        }
    }

    pub fn update(&mut self) {
        let mut i = 0;
        while i < self.x.len() {
            let mut merged = false;
            let mut j = i + 1;
            while j < self.x.len() {
                let dir_x = self.x[i] - self.x[j];
                let dir_y = self.y[i] - self.y[j];
                let distance = (dir_x.powi(2) + dir_y.powi(2)).sqrt();
                if distance < 5.0 {
                    let mid_x = (self.x[i] + self.x[j]) / 2.0;
                    let mid_y = (self.y[i] + self.y[j]) / 2.0;
                    let mid_vel_x = self.vel_x[i] + self.vel_x[j];
                    let mid_vel_y = self.vel_y[i] + self.vel_y[j];
                    let new_mass = self.m[i] + self.m[j];

                    self.x.remove(j);
                    self.y.remove(j);
                    self.vel_x.remove(j);
                    self.vel_y.remove(j);
                    self.m.remove(j);

                    self.x[i] = mid_x;
                    self.y[i] = mid_y;
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

        let mut temp_x = self.x.to_vec();
        let mut temp_y = self.y.to_vec();

        for current in 0..self.x.len() {
            let mut vel_x = self.vel_x[current];
            let mut vel_y = self.vel_y[current];

            for n in 0..self.x.len() {
                if n == current {
                    continue;
                }
                let mut dir_x = self.x[n] - self.x[current];
                let mut dir_y = self.y[n] - self.y[current];
                let distance = (dir_x.powi(2) + dir_y.powi(2)).sqrt();
                dir_x /= distance;
                dir_y /= distance;
                let force = (self.m[current] * self.m[n]) / distance.powi(2);
                let acceleration = force / self.m[current];
                vel_x += dir_x * acceleration;
                vel_y += dir_y * acceleration;
            }
            temp_x[current] += vel_x;
            temp_y[current] += vel_y;
            self.vel_x[current] = vel_x;
            self.vel_y[current] = vel_y;
        }
        self.x = temp_x.to_vec();
        self.y = temp_y.to_vec();
    }
}

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Gravity Particle Sim", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut particles = Particles::new(1000);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.set_draw_color(Color::WHITE);
        for particle in 0..particles.x.len() {
            canvas
                .draw_point(Point::new(
                    particles.x[particle] as i32,
                    particles.y[particle] as i32,
                ))
                .unwrap();
        }

        canvas.present();
        particles.update();
        ::std::thread::sleep(Duration::from_millis(16));
    }
}
