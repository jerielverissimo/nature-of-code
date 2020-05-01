use std::error::Error;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render;
use sdl2::video;

use super::walker::Walker;

pub const WINDOW_WIDTH: u32 = 640;
pub const WINDOW_HEIGHT: u32 = 360;

pub struct App {
    pub canvas: render::Canvas<video::Window>,
    events: sdl2::EventPump,
    w: Walker,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Nature of Code", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()?;

        let mut canvas = window.into_canvas().build()?;
        let events = sdl_context.event_pump()?;

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();

        Ok(Self {
            canvas,
            events,
            w: Walker::new(),
        })
    }

    pub fn run(&mut self) {
        'running: loop {
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            self.draw();

            self.canvas.present();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        }
    }

    pub fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        self.w.step();
        self.w.clone().display(self)?;
        Ok(())
    }
}
