use super::app::{App, WINDOW_HEIGHT, WINDOW_WIDTH};

use std::error::Error;

use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Copy, Clone)]
pub struct Walker {
    x: i32,
    y: i32,
}

impl Walker {
    pub fn new() -> Self {
        let x = (WINDOW_WIDTH / 2) as i32;
        let y = (WINDOW_HEIGHT / 2) as i32;

        Self { x, y }
    }

    pub fn display(&self, app: &mut App) -> Result<(), Box<dyn Error>> {
        let r = Rect::new(self.x, self.y, 2, 2);
        app.canvas.set_draw_color(Color::RGB(0, 0, 0));
        app.canvas.fill_rect(r)?;
        app.canvas.draw_rect(r)?;
        Ok(())
    }

    pub fn step(&mut self) {
        let mut rng = rand::thread_rng();
        let stepx: i32 = rng.gen_range(0, 3) - 1;
        let stepy: i32 = rng.gen_range(0, 3) - 1;

        self.x += stepx;
        self.y += stepy;
    }
}
