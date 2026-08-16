use rayon::prelude::*;
use std::f32::consts::{PI, TAU};
use std::time::Duration;

use rand::prelude::*;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Point;
use sdl3::render::{Canvas, FPoint};
use sdl3::video::Window;

const PARTICLE_COUNT: usize = 10000;
const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const G: f32 = 0.01;
const ANGULAR_VELOCITY: f32 = 0.01;
const DISK_DENSITY_DISTRIBUTION: f32 = 1.3;

fn draw_circle(render: &mut Canvas<Window>, pos_x: f32, pos_y: f32, radius: f32) {
    render.set_draw_color(Color::WHITE);
    if radius <= 2.0 {
        render.draw_point(FPoint::new(pos_x, pos_y)).unwrap();
        return;
    }
    let diameter = radius * 2.0;

    let mut x = radius - 1.0;
    let mut y = 0.0;
    let mut tx = 1.0;
    let mut ty = 1.0;
    let mut error = tx - diameter;

    while x >= y {
        render
            .draw_point(FPoint::new(pos_x + x, pos_y - y))
            .unwrap();
        render
            .draw_point(FPoint::new(pos_x + x, pos_y + y))
            .unwrap();
        render
            .draw_point(FPoint::new(pos_x - x, pos_y - y))
            .unwrap();
        render
            .draw_point(FPoint::new(pos_x - x, pos_y + y))
            .unwrap();
        render
            .draw_point(FPoint::new(pos_x + y, pos_y - x))
            .unwrap();
        render
            .draw_point(FPoint::new(pos_x + y, pos_y + x))
            .unwrap();
        render
            .draw_point(FPoint::new(pos_x - y, pos_y - x))
            .unwrap();
        render
            .draw_point(FPoint::new(pos_x - y, pos_y + x))
            .unwrap();

        if error <= 0.0 {
            y += 1.0;
            error += ty;
            ty += 2.0;
        }

        if error > 0.0 {
            x -= 1.0;
            tx += 2.0;
            error += tx - diameter;
        }
    }
}

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
        let mut rng = rand::rng();
        let mut temp_x = Vec::new();
        let mut temp_y = Vec::new();
        let mut temp_vel_x: Vec<f32> = Vec::new();
        let mut temp_vel_y: Vec<f32> = Vec::new();
        let mut temp_m: Vec<f32> = Vec::new();
        for n in 0..num {
            let r = HEIGHT as f32 / 4.0 * rng.random::<f32>().powf(DISK_DENSITY_DISTRIBUTION);
            let theta = rng.random_range(0.0..TAU);
            temp_x.push(WIDTH as f32 / 2.0 + r * theta.cos());
            temp_y.push(HEIGHT as f32 / 2.0 + r * theta.sin());
            temp_vel_x.push((-temp_y[n] + WIDTH as f32 / 2.0) * ANGULAR_VELOCITY);
            temp_vel_y.push((temp_x[n] - HEIGHT as f32 / 2.0) * ANGULAR_VELOCITY);
            temp_m.push(rng.random_range(1.0..2.0));
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
        let mut rng = rand::rng();
        let mut temp_x = Vec::new();
        let mut temp_y = Vec::new();
        let mut temp_vel_x: Vec<f32> = Vec::new();
        let mut temp_vel_y: Vec<f32> = Vec::new();
        let mut temp_m: Vec<f32> = Vec::new();
        for n in 0..self.num {
            let r = HEIGHT as f32 / 4.0 * rng.random::<f32>().powf(DISK_DENSITY_DISTRIBUTION);
            let theta = rng.random_range(0.0..TAU);
            temp_x.push(WIDTH as f32 / 2.0 + r * theta.cos());
            temp_y.push(HEIGHT as f32 / 2.0 + r * theta.sin());
            temp_vel_x.push((-temp_y[n] + WIDTH as f32 / 2.0) * ANGULAR_VELOCITY);
            temp_vel_y.push((temp_x[n] - HEIGHT as f32 / 2.0) * ANGULAR_VELOCITY);
            temp_m.push(rng.random_range(1.0..2.0));
        }
        self.x = temp_x;
        self.y = temp_y;
        self.vel_x = temp_vel_x;
        self.vel_y = temp_vel_y;
        self.m = temp_m;
    }

    pub fn update(&mut self) {
        // collision detection and merging
        let mut i = 0;
        while i < self.x.len() {
            let mut merged = false;
            let mut j = i + 1;
            while j < self.x.len() {
                let dir_x = self.x[i] - self.x[j];
                let dir_y = self.y[i] - self.y[j];
                let distance = (dir_x.powi(2) + dir_y.powi(2)).sqrt();
                let radius = ((self.m[i] * 3.0) / (4.0 * PI)).cbrt();
                if distance < radius {
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
                    let distance = (dir_x.powi(2) + dir_y.powi(2)).sqrt();
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

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Gravity Particle Sim", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut particles = Particles::new(PARTICLE_COUNT);
    let mut trail_grid = vec![vec![0; WIDTH]; HEIGHT];
    let trail_lenght = 100;

    let mut view_off_x = 0;
    let mut view_off_y = 0;

    let mut run = false;
    let mut update_intervall: u64 = 5;

    'running: loop {
        let star_time = ::std::time::Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => run = !run,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    particles.restart();
                    view_off_x = 0;
                    view_off_y = 0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => view_off_x += 10,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => view_off_x -= 10,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => view_off_y += 10,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => view_off_y -= 10,
                Event::KeyDown {
                    keycode: Some(Keycode::KpPlus),
                    ..
                } => {
                    if update_intervall >= 2 {
                        update_intervall -= 1
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::KpMinus),
                    ..
                } => update_intervall += 1,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Draw particle Trails
        canvas.set_draw_color(Color::RGB(128, 0, 0));
        for (ny, y) in trail_grid.iter_mut().enumerate() {
            for (nx, x) in y.iter_mut().enumerate() {
                if *x >= 1 {
                    canvas.draw_point(Point::new(nx as i32, ny as i32)).unwrap();
                }
                if run {
                    *x -= 1;
                }
            }
        }

        // Draw Particles
        canvas.set_draw_color(Color::WHITE);
        for particle in 0..particles.x.len() {
            let radius = ((particles.m[particle] * 3.0) / (4.0 * PI)).cbrt();
            draw_circle(
                &mut canvas,
                particles.x[particle] + view_off_x as f32,
                particles.y[particle] + view_off_y as f32,
                radius + 1.0,
            );
            if (particles.x[particle] + view_off_x as f32) < WIDTH as f32
                && (particles.y[particle] + view_off_y as f32) < HEIGHT as f32
            {
                trail_grid[(particles.y[particle] + view_off_y as f32) as usize]
                    [(particles.x[particle] + view_off_x as f32) as usize] = trail_lenght;
            }
        }

        canvas.present();
        if run {
            particles.update();
        }

        let elapsed = ::std::time::Instant::now() - star_time;
        if elapsed < Duration::from_millis(update_intervall) {
            ::std::thread::sleep(Duration::from_millis(update_intervall) - elapsed);
        }
    }
}
