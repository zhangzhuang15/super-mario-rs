pub(crate) struct MenuOption {
    pub text: String,
    pub x_pos: i32,
    pub y_pos: i32,
}

impl MenuOption {
    pub fn from(text: String, x_pos: i32, y_pos: i32) -> MenuOption {
        MenuOption { text, x_pos, y_pos }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}
