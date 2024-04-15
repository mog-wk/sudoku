#![allow(unused)]

mod error;
mod ui;
mod utils;

use error::Error;
use ui::clickbox;
use utils::window;

use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// dev
use sdl2::pixels::Color;

// TODO: Proper err
fn main() -> Result<(), Error> {
    let sdl_ctx = sdl2::init()?;
    let ttf_ctx = sdl2::ttf::init().unwrap();

    let video = sdl_ctx.video()?;

    let (width, height) = (window::Config::WIDTH, window::Config::HEIGHT);
    let app_window = video
        .window("sudoku-dev", width, height)
        .position(0, 0)
        .resizable()
        .opengl()
        .build()?;

    let mut canvas = app_window
        .into_canvas()
        //.present_vsync()
        .build()?;

    let texture_creator = canvas.texture_creator();

    // init font
    let mut font_regular = ttf_ctx.load_font(
        std::path::Path::new(&"./fonts/OpenSans/OpenSans-Regular.ttf"),
        128,
    )?;
    let mut font_bold = ttf_ctx.load_font(
        std::path::Path::new(&"./fonts/OpenSans/OpenSans-Bold.ttf"),
        128,
    )?;

    // dev
    let dev_box = clickbox::new().with_pos(20, 20).with_dim(50, 50).build()?;
    //dev_box.exec();

    println!("{:?}", dev_box);

    let mut event_pump = sdl_ctx.event_pump()?;

    'app_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'app_loop,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'app_loop, // dev
                _ => (),
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // ========== Render testing ===============

        utils::render::text(
            &mut canvas,
            &texture_creator,
            &font_bold,
            Color::RGB(127, 127, 127),
            "Bold Text".to_string(),
            (16, 16, 256, 64),
        )?;
        utils::render::text(
            &mut canvas,
            &texture_creator,
            &font_regular,
            Color::RGB(255, 255, 255),
            "Regular Text".to_string(),
            (16, 16 + 80, 256, 64),
        )?;

        canvas.present();

        sleep(Duration::new(0, 1_000_000_000_u32 / 60));
    }

    Ok(())
}
