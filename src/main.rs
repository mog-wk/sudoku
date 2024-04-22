#![allow(unused)]

mod error;
mod lib;
mod sudoku;
mod ui;
mod utils;

use error::Error;
use ui::clickbox;
use utils::window;

use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;

use crate::lib::AppState;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use utils::input;

// dev
use sdl2::pixels::Color;

use crate::ui::clickbox::ClickBox;

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

    let mut canvas = app_window.into_canvas().build()?;

    let texture_creator = canvas.texture_creator();

    // TODO make objective dir
    //let main_dir = ...

    // init font
    let mut font_regular = ttf_ctx.load_font(
        std::path::Path::new(&"./fonts/OpenSans/OpenSans-Regular.ttf"),
        128,
    )?;
    let mut font_bold = ttf_ctx.load_font(
        std::path::Path::new(&"./fonts/OpenSans/OpenSans-Bold.ttf"),
        128,
    )?;

    let mut menu = ui::menu::MainMenu::dev_test()?;

    let mut event_pump = sdl_ctx.event_pump()?;

    let mut mouse_buffer = HashSet::new();
    let mut cbox_focus: Option<usize> = None;

    let mut app_state: AppState = AppState::Menu;

    'app_loop: loop {
        // handle input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'app_loop,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'app_loop,
                _ => (),
            }
        }

        input::mouse_input(
            &mut menu.input_boxes,
            event_pump.mouse_state(),
            &mouse_buffer,
            &mut cbox_focus,
        );

        match app_state {
            AppState::Menu => {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();

                // ========== Render testing ===============
                menu.render(&mut canvas, &texture_creator, &font_regular)?;
            }
            _ => (),
        }

        canvas.present();

        sleep(Duration::new(0, 1_000_000_000_u32 / 60));
    }

    Ok(())
}
