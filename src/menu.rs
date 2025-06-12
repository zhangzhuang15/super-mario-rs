use sdl2::sys::SDL_Renderer;

use crate::menu_option::MenuOption;

pub(crate) struct Menu {
    pub menu_options: Vec<MenuOption>,
    pub active_menu_option: i32,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            menu_options: vec![],
            active_menu_option: 0,
        }
    }

    pub fn update(&self) {}

    pub fn draw(&self, render: *mut SDL_Renderer) {
        (0..self.menu_options.len()).for_each(|it| {});
    }
}
