use super::clickbox;
use crate::Error;
use sdl2::{
    pixels::Color,
    render::{TextureCreator, WindowCanvas},
    ttf::Font,
    video::WindowContext,
};

pub struct MainMenu {
    pub input_boxes: Vec<clickbox::ClickBox>,
}

impl MainMenu {
    pub fn dev_test() -> Result<Self, Error> {
        let mut dev_box = clickbox::new()
            .with_pos(240, 120)
            .with_dim(240, 40)
            .text("dev_box render test".to_string())
            .fill_color(Color::RGB(127, 127, 0))
            .with_event(|| {
                println!("dev box has it\'s event executed!!");
            })
            .build()?;
        let input_boxes = vec![dev_box];
        Ok(Self { input_boxes })
    }

    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font,
    ) -> Result<(), Error> {
        for cbox in self.input_boxes.iter() {
            cbox.render(canvas, texture_creator, font)?;
        }
        Ok(())
    }
}
