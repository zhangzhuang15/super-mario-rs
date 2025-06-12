use sdl2::sys::SDL_Rect;

use crate::menu_option::MenuOption;

pub(crate) struct MainMenu {
    pub select_world: bool,
    pub active_world_id: i32,
    pub active_second_world_id: i32,
    pub select_world_rect: SDL_Rect,
    pub menu_options: Vec<MenuOption>,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        let mut v = MainMenu {
            select_world: false,
            active_world_id: 0,
            active_second_world_id: 0,
            select_world_rect: SDL_Rect {
                h: 72,
                x: 122,
                y: 280,
                w: 306,
            },
            menu_options: vec![],
        };

        v.menu_options
            .push(MenuOption::from("1 PLAYER GAME".to_string(), 178, 276));

        v.menu_options
            .push(MenuOption::from("OPTIONS".to_string(), 222, 308));

        v.menu_options
            .push(MenuOption::from("ABOUT".to_string(), 237, 340));

        v
    }
}
