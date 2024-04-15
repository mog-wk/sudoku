use crate::error::Error;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

#[derive(Debug)]
pub struct ClickBox {
    pos: (i32, i32),
    dim: (u32, u32),
    fill_color: Color,
    border_color: Color,
    text_color: Color,
    text: String,
    event: fn() -> (),
}

impl ClickBox {
    pub fn new() -> ClickBoxBuilder {
        ClickBoxBuilder::new()
    }
    pub fn exec(&self) -> () {
        (self.event)();
    }
    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), Error> {
        let rt = Rect::new(self.pos.0, self.pos.1, self.dim.0, self.dim.1);
        canvas.set_draw_color(self.border_color);
        canvas.set_draw_color(self.fill_color);
        canvas.fill_rect(rt)?;
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
    event: Option<fn() -> ()>,
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
    pub fn with_event(mut self, event: fn() -> ()) -> Self {
        self.event = Some(event);
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

        let text = self.text.unwrap_or("".to_string());

        let event = self.event.unwrap_or(|| ());

        Ok(ClickBox {
            pos,
            dim,
            fill_color,
            border_color,
            text_color,
            text,
            event,
        })
    }
}

pub fn new() -> ClickBoxBuilder {
    ClickBoxBuilder::new()
}
