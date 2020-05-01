use std::error::Error;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Nature of Code", 800, 600)
        .position_centered()
        .opengl()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

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

        canvas.clear();
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
