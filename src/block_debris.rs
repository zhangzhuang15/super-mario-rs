use crate::map::Map;
use sdl2::sys::SDL_Renderer;
use std::sync::{Arc, Mutex};

use crate::cfg::GAME_HEIGHT;

pub enum DebrisState {
    Animation,
    Delete,
}

pub struct Position(pub i32, pub i32);

pub(crate) struct BlockDebris {
    debris_state: DebrisState,
    position_l: Position,
    position_r: Position,
    position_l2: Position,
    position_r2: Position,

    frame_id: i32,
    speed_x: f32,
    speed_y: f32,
    rotate: bool,
}

impl BlockDebris {
    pub fn new() -> BlockDebris {
        BlockDebris {
            debris_state: DebrisState::Animation,
            position_l: Position(0, 0),
            position_r: Position(16, 0),
            position_l2: Position(0, 16),
            position_r2: Position(16, 16),
            frame_id: 0,
            speed_x: 2.15,
            speed_y: 1.0,
            rotate: false,
        }
    }

    pub fn from(x_pos: i32, y_pos: i32) -> BlockDebris {
        let mut t = BlockDebris::new();
        t.position_l = Position(x_pos, y_pos);
        t.position_r = Position(x_pos + 16, y_pos);
        t.position_l2 = Position(x_pos, y_pos + 16);
        t.position_r2 = Position(x_pos + 16, y_pos + 16);
        t
    }

    pub fn update(&mut self) {
        self.frame_id += 1;

        if self.frame_id > 4 {
            self.rotate = !self.rotate;
            self.frame_id = 0;
        }

        let update_x_y_1 = |old_x: i32, old_y: i32| -> (i32, i32) {
            let current_x = (old_x as f32 - self.speed_x) as i32;
            let mut current_y = (old_y as f32 + self.speed_y - 3 as f32) as i32;
            if self.speed_y < 2.5 {
                current_y -= 3;
            } else if self.speed_y < 3.0 {
                current_y = (current_y as f32 - 1.5) as i32;
            } else if self.speed_y < 3.5 {
                current_y = (current_y as f32 + 1.5) as i32;
            } else {
                current_y += 3;
            }
            (current_x, current_y)
        };

        let update_x_y_2 = |old_x: i32, old_y: i32| -> (i32, i32) {
            let current_x = (old_x as f32 + self.speed_x - 1.1) as i32;
            let mut current_y = (old_y as f32 + self.speed_y - 1.5) as i32;
            if self.speed_y < 1.3 {
                current_y -= 3;
            } else if self.speed_y < 1.5 {
                current_y = (current_y as f32 - 1.0) as i32;
            } else if self.speed_y < 1.8 {
                current_y = (current_y as f32 + 1.0) as i32;
            } else {
                current_y += 3;
            }
            (current_x, current_y)
        };

        {
            let (x, y) = update_x_y_1(self.position_l.0, self.position_l.1);
            self.position_l = Position(x, y);

            let (x, y) = update_x_y_1(self.position_r.0, self.position_r.1);
            self.position_r = Position(x, y);
        }

        {
            let (x, y) = update_x_y_2(self.position_l2.0, self.position_l2.1);
            self.position_l2 = Position(x, y);

            let (x, y) = update_x_y_2(self.position_r2.0, self.position_r2.1);
            self.position_r2 = Position(x, y);
        }

        self.speed_x += 1.09;
        self.speed_y += 1.005;

        if self.position_l.1 >= GAME_HEIGHT {
            self.debris_state = DebrisState::Delete;
        }
    }

    pub fn draw(&mut self, map: &mut Map, render: *mut SDL_Renderer) {
        let arc_map = map;
        let id = {
            let current_type = arc_map.current_level_type();
            if current_type == 0 || current_type == 4 {
                64
            } else if current_type == 1 {
                65
            } else {
                66
            }
        };
        let x_pos = arc_map.x_pos;

        let block = arc_map.get_block_mut(id);
        let texture = block.sprite.get_current_texture();

        texture.draw(
            render,
            self.position_l.0 + x_pos as i32,
            self.position_l.1,
            self.rotate,
        );

        texture.draw(
            render,
            self.position_r.0 + x_pos as i32,
            self.position_r.1,
            self.rotate,
        );

        texture.draw(
            render,
            self.position_l2.0 + x_pos as i32,
            self.position_l2.1,
            self.rotate,
        );

        texture.draw(
            render,
            self.position_r2.0 + x_pos as i32,
            self.position_r2.1,
            self.rotate,
        );
    }
}
