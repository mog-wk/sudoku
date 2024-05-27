use super::{clickbox, funcs, readbox};
use crate::Error;
use sdl2::{
    pixels::Color,
    render::{TextureCreator, WindowCanvas},
    ttf::Font,
    video::WindowContext,
};

use crate::utils::window::Config as window;

pub struct MainMenu {
    pub read_boxes: Vec<readbox::Readbox>,
    pub input_boxes: Vec<clickbox::ClickBox>,
}

impl MainMenu {
    const BOX_ALIGN_X: i32 = window::WIDTH_UNIT * 1;
    const BOX_ALIGN_Y: i32 = window::HEIGHT_UNIT * 3;
    const BOX_WIDTH: u32 = window::WIDTH_UNIT as u32 * 2;
    const BOX_HEIGHT: u32 = window::HEIGHT_UNIT as u32 * 1;
    const TEXTLEN: usize = 10;

    pub fn dev_test() -> Result<Self, Error> {
        let mut dev_box = clickbox::new()
            .with_pos(240, 120)
            .with_dim(240, 40)
            .text("dev_box render test".to_string())
            .fill_color(Color::RGB(127, 127, 0))
            .with_event(|| {
                println!("dev box has it\'s event executed!!");
                sudoku::AppState::DEVTEST
            })
            .build()?;
        let read_boxes = vec![];
        let input_boxes = vec![dev_box];
        Ok(Self {
            read_boxes,
            input_boxes,
        })
    }

    pub fn new() -> Result<Self, Error> {
        let y_offset = window::HEIGHT_UNIT + 8;

        // title

        let title_box = readbox::new(
            (window::WIDTH_UNIT * 3, window::HEIGHT_UNIT * 1), // position
            (
                window::WIDTH_UNIT as u32 * 2,
                window::HEIGHT_UNIT as u32 * 1,
            ), // dimention
            "Sudoku".to_string(),
        );

        // buttons

        let mut to_game_box = clickbox::new()
            .with_pos(Self::BOX_ALIGN_X, Self::BOX_ALIGN_Y)
            .with_dim(Self::BOX_WIDTH, Self::BOX_HEIGHT)
            .with_textlen(Self::TEXTLEN)
            .text("Play".to_string())
            .fill_color(Color::RGB(127, 0, 0))
            .with_event(funcs::game)
            .build()?;
        let mut to_setting_box = clickbox::new()
            .with_pos(Self::BOX_ALIGN_X, Self::BOX_ALIGN_Y + y_offset)
            .with_dim(Self::BOX_WIDTH, Self::BOX_HEIGHT)
            .with_textlen(Self::TEXTLEN)
            .text("Settings".to_string())
            .fill_color(Color::RGB(127, 0, 0))
            .with_event(funcs::settings)
            .build()?;
        let mut to_about_box = clickbox::new()
            .with_pos(Self::BOX_ALIGN_X, Self::BOX_ALIGN_Y + y_offset * 2)
            .with_dim(Self::BOX_WIDTH, Self::BOX_HEIGHT)
            .with_textlen(Self::TEXTLEN)
            .text("About".to_string())
            .fill_color(Color::RGB(127, 0, 0))
            .with_event(funcs::about)
            .build()?;
        let mut to_quit_box = clickbox::new()
            .with_pos(Self::BOX_ALIGN_X, Self::BOX_ALIGN_Y + y_offset * 3)
            .with_dim(Self::BOX_WIDTH, Self::BOX_HEIGHT)
            .with_textlen(Self::TEXTLEN)
            .text("Quit".to_string())
            .fill_color(Color::RGB(127, 0, 0))
            .with_event(funcs::quit)
            .build()?;

        Ok(Self {
            read_boxes: vec![title_box],
            input_boxes: vec![to_game_box, to_setting_box, to_about_box, to_quit_box],
        })
    }

    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font,
    ) -> Result<(), Error> {
        for rbox in self.read_boxes.iter() {
            rbox.render(canvas, texture_creator, font)?;
        }
        for cbox in self.input_boxes.iter() {
            cbox.render(canvas, texture_creator, font)?;
        }
        Ok(())
    }
}
