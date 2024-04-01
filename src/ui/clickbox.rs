use crate::error::Error;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

#[derive(Debug)]
pub struct ClickBox {
    pos: (i32, i32),
    dim: (u32, u32),
    bg: Color,
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
}

#[derive(Debug)]
pub struct ClickBoxBuilder {
    pos: Option<(i32, i32)>,
    dim: Option<(u32, u32)>,
    bg: Option<Color>,
    text: Option<String>,
    event: Option<fn() -> ()>,
}


impl ClickBoxBuilder {
    pub fn new() -> Self {
        Self {
            pos: None,
            dim: None,
            bg: None,
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
        let pos = self.pos.ok_or(Error::Generic("invalid position for click box".to_string()))?;
        let dim = self.dim.ok_or(Error::Generic("invalid dimentions for click box".to_string()))?;
        let bg = self.bg.unwrap_or(Color::RGB(0, 0, 0));

        let text = self.text.unwrap_or("".to_string());

        let event = self.event.unwrap_or(|| ());

        Ok(ClickBox {
            pos,
            dim,
            bg,
            text,
            event,
        })
    }
}

pub fn new() -> ClickBoxBuilder {
    ClickBoxBuilder::new()
}
