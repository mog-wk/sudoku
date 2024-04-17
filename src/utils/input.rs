use crate::ui::clickbox::ClickBox;
use sdl2::mouse::MouseState;

pub fn mouse_input(ui_elements: &Vec<&ClickBox>, x: i32, y: i32) {
    for cbox in ui_elements {
        if cbox.contains_point((x, y)) {
            cbox.click_event();
        }
    }
}
