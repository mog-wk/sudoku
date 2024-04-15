use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::error::Error;

pub fn text(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    color: Color,
    text: String,
    (x, y, w, h): (i32, i32, u32, u32),
) -> Result<(), Error> {
    // test
    let surface = font.render(&text).blended(color)?;

    let texture = texture_creator.create_texture_from_surface(&surface)?;

    let target = Rect::new(x, y, w, h);
    canvas.copy(&texture, None, Some(target))?;
    Ok(())
}

//#[cfg(test)]
//#[path = "../tests/render_text.rs"]
//mod tests;
