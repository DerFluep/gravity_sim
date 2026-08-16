use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use sdl3::EventPump;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Point;
use sdl3::render::Canvas;
use sdl3::video::Window;

use crate::{HEIGHT, WIDTH};

pub struct Viewport {
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

impl Viewport {
    pub fn new() -> Viewport {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Game of life", WIDTH as u32, HEIGHT as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas();
        let event_pump = sdl_context.event_pump().unwrap();
        Viewport { canvas, event_pump }
    }

    fn get_input(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                _ => {}
            }
        }

        false
    }

    pub fn draw(&mut self, block: &Block, quit: Arc<AtomicBool>) {
        let interval = Duration::from_micros(1000000 / 60);
        'running: loop {
            let before = Instant::now();

            if self.get_input() {
                quit.store(true, Ordering::Relaxed);
                break 'running;
            }

            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();

            self.canvas.set_draw_color(Color::RGB(255, 0, 0));

            for n in 0..3 {
                self.canvas
                    .draw_line(
                        Point::new(
                            (block.get_center().get_x() + block.get_vertex(n).get_x()) as i32,
                            (block.get_center().get_z() + block.get_vertex(n).get_z()) as i32,
                        ),
                        Point::new(
                            (block.get_center().get_x() + block.get_vertex(n + 1).get_x()) as i32,
                            (block.get_center().get_z() + block.get_vertex(n + 1).get_z()) as i32,
                        ),
                    )
                    .unwrap();
            }
            self.canvas
                .draw_line(
                    Point::new(
                        (block.get_center().get_x() + block.get_vertex(3).get_x()) as i32,
                        (block.get_center().get_z() + block.get_vertex(3).get_z()) as i32,
                    ),
                    Point::new(
                        (block.get_center().get_x() + block.get_vertex(0).get_x()) as i32,
                        (block.get_center().get_z() + block.get_vertex(0).get_z()) as i32,
                    ),
                )
                .unwrap();

            for n in 4..7 {
                self.canvas
                    .draw_line(
                        Point::new(
                            (block.get_center().get_x() + block.get_vertex(n).get_x()) as i32,
                            (block.get_center().get_z() + block.get_vertex(n).get_z()) as i32,
                        ),
                        Point::new(
                            (block.get_center().get_x() + block.get_vertex(n + 1).get_x()) as i32,
                            (block.get_center().get_z() + block.get_vertex(n + 1).get_z()) as i32,
                        ),
                    )
                    .unwrap();
            }
            self.canvas
                .draw_line(
                    Point::new(
                        (block.get_center().get_x() + block.get_vertex(7).get_x()) as i32,
                        (block.get_center().get_z() + block.get_vertex(7).get_z()) as i32,
                    ),
                    Point::new(
                        (block.get_center().get_x() + block.get_vertex(4).get_x()) as i32,
                        (block.get_center().get_z() + block.get_vertex(4).get_z()) as i32,
                    ),
                )
                .unwrap();

            for n in 0..4 {
                self.canvas
                    .draw_line(
                        Point::new(
                            (block.get_center().get_x() + block.get_vertex(n).get_x()) as i32,
                            (block.get_center().get_z() + block.get_vertex(n).get_z()) as i32,
                        ),
                        Point::new(
                            (block.get_center().get_x() + block.get_vertex(n + 4).get_x()) as i32,
                            (block.get_center().get_z() + block.get_vertex(n + 4).get_z()) as i32,
                        ),
                    )
                    .unwrap();
            }

            self.canvas.present();

            let elapsed = Instant::now() - before;
            if elapsed < interval {
                ::std::thread::sleep(interval - elapsed);
            }
        }
    }
}
