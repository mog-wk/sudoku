
mod utils;

use utils::window;

use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// dev
use sdl2::pixels::Color;


// TODO: Proper err
fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init().map_err(|e| e.to_string())?;

    let video = sdl_ctx.video()?;

    let (width, height) = (window::Config::WIDTH, window::Config::HEIGHT);
    let app_window = video.window("sudoku-dev", width, height)
        .position(0, 0)
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = app_window.into_canvas()
        //.present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    // dev
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_ctx.event_pump().map_err(|e| e.to_string())?;

    'app_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'app_loop,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'app_loop, // dev
                _ => (),
            }
        }

        sleep(Duration::new(0, 1_000_000_000_u32 / 60));
    }

    Ok(())
}
