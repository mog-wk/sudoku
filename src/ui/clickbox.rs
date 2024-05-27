use crate::error::Error;
use crate::utils::render;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use sudoku::AppState;

#[derive(Debug)]
pub struct ClickBox {
    pos: (i32, i32),
    dim: (u32, u32),
    fill_color: Color,
    border_color: Color,
    text_color: Color,
    text: String,
    clicked: bool,
    event: fn() -> AppState,
}

impl ClickBox {
    pub fn new() -> ClickBoxBuilder {
        ClickBoxBuilder::new()
    }
    pub fn exec(&self) -> AppState {
        (self.event)()
    }
    pub fn click_event(&self) -> AppState {
        self.exec()
    }
    pub fn set_clicked(&mut self, val: bool) {
        self.clicked = val;
    }
    pub fn contains_point(&self, p: (i32, i32)) -> bool {
        p.0 > self.pos.0
            && p.0 < self.pos.0 + self.dim.0 as i32
            && p.1 > self.pos.1
            && p.1 < self.pos.1 + self.dim.1 as i32
    }
    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        txc: &TextureCreator<WindowContext>,
        font: &Font,
    ) -> Result<(), Error> {
        let rt = Rect::new(self.pos.0, self.pos.1, self.dim.0, self.dim.1);
        canvas.set_draw_color(self.border_color);
        // TODO BORDER COLOR
        canvas.set_draw_color(self.fill_color);
        canvas.fill_rect(rt)?;

        let limits = (self.pos.0, self.pos.1, self.dim.0, self.dim.1);
        render::text(canvas, txc, font, self.text_color, &self.text, limits)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ClickBoxBuilder {
    pos: Option<(i32, i32)>,
    dim: Option<(u32, u32)>,

    fill_color: Option<Color>,
    border_color: Option<Color>,
    text_color: Option<Color>,

    text: Option<String>,
    textlen: Option<usize>,
    event: Option<fn() -> AppState>,
}

impl ClickBoxBuilder {
    pub fn new() -> Self {
        Self {
            pos: None,
            dim: None,
            fill_color: None,
            border_color: None,
            text_color: None,
            text: None,
            textlen: None,
            event: None,
        }
    }
    pub fn with_pos(mut self, x: i32, y: i32) -> Self {
        self.pos = Some((x, y));
        self
    }
    pub fn with_dim(mut self, w: u32, h: u32) -> Self {
        self.dim = Some((w, h));
        self
    }
    pub fn with_textlen(mut self, len: usize) -> Self {
        self.textlen = Some(len);
        self
    }
    pub fn with_event(mut self, event: fn() -> AppState) -> Self {
        self.event = Some(event);
        self
    }
    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }
    pub fn text(mut self, text: String) -> Self {
        if let Some(len) = self.textlen {
            // append text with spaces to match textlen
            let diff = len - text.len();
            let mut st = String::from(text);
            for _ in 0..diff {
                st.push(' ');
            }
            self.text = Some(st);
        } else {
            self.textlen = Some(text.len());
            self.text = Some(text);
        }
        self
    }

    pub fn build(self) -> Result<ClickBox, Error> {
        let pos = self
            .pos
            .ok_or(Error::Generic("invalid position for click box".to_string()))?;
        let dim = self.dim.ok_or(Error::Generic(
            "invalid dimentions for click box".to_string(),
        ))?;
        let fill_color = self.fill_color.unwrap_or(Color::RGBA(0, 0, 0, 0));
        let border_color = self.border_color.unwrap_or(Color::RGBA(0, 0, 0, 0));
        let text_color = self.text_color.unwrap_or(Color::RGB(155, 155, 155));

        let text = self
            .text
            .ok_or(Error::Generic("Invalid text".to_string()))?;

        let event = self.event.unwrap();

        Ok(ClickBox {
            pos,
            dim,
            fill_color,
            border_color,
            text_color,
            text,
            clicked: false,
            event,
        })
    }
}

pub fn new() -> ClickBoxBuilder {
    ClickBoxBuilder::new()
}
