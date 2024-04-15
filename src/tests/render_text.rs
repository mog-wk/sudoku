use super::render_text;
use std::path::Path;

#[test]
fn window() {
    let sdl_ctx = sdl2::init().unwrap();

    let ttf_ctx = sdl2::ttf::init().unwrap();
    // unable to init font
    //let mut font = ttf_ctx
    //.load_font(Path::new(&"../../fonts/OpenSans-Regular.ttf"), 64)
    //.unwrap();

    let window = sdl_ctx
        .video()
        .unwrap()
        .window("text render test", 600, 400)
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let tex_creator = canvas.texture_creator();
    let mut event_pump = sdl_ctx.event_pump().unwrap();
    todo!();
    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'run,
                sdl2::event::Event::KeyDown { .. } => {
                    render_text(
                        &mut canvas,
                        &tex_creator,
                        //&font,
                        &format!("{:?}", event),
                        12,
                        12,
                        40,
                        40,
                    )
                    .unwrap();
                }
                _ => (),
            }
        }
    }
}
