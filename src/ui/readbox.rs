use crate::error::Error;
use crate::utils::render;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

pub fn new(pos: (i32, i32), dim: (u32, u32), text: String) -> Readbox {
    Readbox::new(pos, dim, text)
}

pub struct Readbox {
    pos: (i32, i32),
    dim: (u32, u32),
    fill_color: Color,
    text_color: Color,
    text: String,
}

impl Readbox {
    pub fn new(pos: (i32, i32), dim: (u32, u32), text: String) -> Self {
        Self {
            pos,
            dim,
            fill_color: Color::RGBA(0, 0, 0, 255),
            text_color: Color::RGB(222, 222, 222),
            text,
        }
    }
    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        txc: &TextureCreator<WindowContext>,
        font: &Font,
    ) -> Result<(), Error> {
        let rt = Rect::new(self.pos.0, self.pos.1, self.dim.0, self.dim.1);
        canvas.set_draw_color(self.fill_color);
        canvas.fill_rect(rt)?;

        let limits = (self.pos.0, self.pos.1, self.dim.0, self.dim.1);
        render::text(canvas, txc, font, self.text_color, &self.text, limits)?;
        Ok(())
    }
}
