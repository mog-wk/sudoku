use crate::ui::clickbox::ClickBox;
use sdl2::mouse::MouseButton;
use sdl2::mouse::MouseState;

use std::collections::HashSet;

pub fn mouse_input(
    ui_elements: &mut Vec<&mut ClickBox>,
    mouse_state: MouseState,
    mouse_buffer: &HashSet<MouseButton>,
    clickbox_focus: &mut Option<usize>,
) {
    // TODO refactor into struct
    let (x, y) = (mouse_state.x(), mouse_state.y());
    let mouse_buttons = mouse_state.pressed_mouse_buttons().collect();
    let pressed_buttons = &mouse_buttons - mouse_buffer;
    let released_buttons = mouse_buffer - &mouse_buttons;

    for button in pressed_buttons {
        match button {
            MouseButton::Left => {
                for (i, cbox) in ui_elements.iter_mut().enumerate() {
                    if cbox.contains_point((x, y)) {
                        cbox.click_event();
                        cbox.set_clicked(true);
                        *clickbox_focus = Some(i);
                        println!("{:?}", cbox);
                    }
                }
            }
            _ => println!("{:?}", button),
            //_ => if cbox_buffer.is_some() {},
        };
    }
    for button in released_buttons {
        match button {
            MouseButton::Left => {
                match clickbox_focus {
                    Some(i) => {
                        ui_elements[*i].set_clicked(false);
                        *clickbox_focus = None;
                    }
                    None => (),
                };
            }
            _ => println!("{:?}", button),
        };
    }
}
