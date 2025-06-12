use sdl2::sys::SDL_Renderer;
use std::sync::{Arc, Mutex};

use crate::block::Block;
use crate::block_debris::{BlockDebris, Position};
use crate::bubble::Bubble;
use crate::cfg::GAME_HEIGHT;
use crate::coin::Coin;
use crate::event::Event;
use crate::flag::Flag;
use crate::level_text::LevelText;
use crate::map_level::MapLevel;
use crate::minion::Minion;
use crate::pipe::Pipe;
use crate::platform::{self, Platform};
use crate::player::Player;
use crate::points::Points;

pub(crate) struct Map {
    pub x_pos: f32,
    pub y_pos: f32,

    block_size: i32,
    block: Vec<Block>,

    v_minion_size: i32,
    v_minion: Vec<Block>,

    map_width: i32,
    map_height: i32,
    map_of_level: Vec<Vec<MapLevel>>,

    block_debris: Vec<BlockDebris>,
    platform: Vec<Platform>,
    level_text: Vec<LevelText>,
    bubble: Vec<Bubble>,

    current_level_id: i32,
    level_type: i32,
    pub underwater: bool,
    spawn_point_id: i32,
    move_map: bool,
    frame_id: i32,
    map_time: i32,
    pub in_event: bool,
    draw_lines: bool,

    event: Event,
    player: Player,
    flag: Option<Flag>,

    minion: Vec<Vec<Minion>>,
    minion_size: i32,

    coin: Vec<Coin>,
    points: Vec<Points>,
    pipe: Vec<Pipe>,
}

impl Map {
    // TODO: find another way making child can access Map instead of Arc and Mutex,
    // cause we dont have multiple threads !
    pub fn new(render: *mut SDL_Renderer) -> Map {
        let map = Map {
            x_pos: 0.0,
            y_pos: 0.0,
            block_size: 0,
            block: Vec::new(),
            v_minion_size: 0,
            v_minion: Vec::new(),
            map_width: 0,
            map_height: 0,
            map_of_level: Vec::new(),
            block_debris: Vec::new(),
            platform: Vec::new(),
            level_text: Vec::new(),
            bubble: Vec::new(),
            current_level_id: 0,
            level_type: 0,
            underwater: false,
            spawn_point_id: 0,
            move_map: false,
            frame_id: 0,
            map_time: 0,
            in_event: false,
            draw_lines: false,
            event: Event::new(),
            player: Player::from(render, 84.0, 368.0),
            flag: None,
            minion: Vec::new(),
            minion_size: 0,
            coin: Vec::new(),
            points: Vec::new(),
            pipe: Vec::new(),
        };

        map
    }

    pub fn get_list_id(&self, x: i32) -> i32 {
        x / 160
    }

    pub fn get_block(&self, id: i32) -> &Block {
        &self.block[id as usize]
    }

    pub fn get_block_mut(&mut self, id: i32) -> &mut Block {
        &mut self.block[id as usize]
    }

    pub fn current_level_type(&self) -> i32 {
        self.level_type
    }

    pub fn get_block_id(&self, x: i32, y: i32) -> Position {
        let x = if x < 0 { 0 } else { (x / 32) as i32 };
        let y = if y > GAME_HEIGHT - 16 {
            0
        } else {
            ((GAME_HEIGHT - 16 - y + 32) / 32) as i32
        };
        Position(x, y)
    }

    pub fn get_block_id_x(&self, x: i32) -> i32 {
        if x < 0 { 0 } else { (x / 32) as i32 }
    }

    pub fn check_collision(&self, v: Position, check_visible: bool) -> bool {
        let block = &self.block[self.map_of_level[v.0 as usize][v.1 as usize].block_id as usize];
        let out = block.is_collision()
            && (if check_visible {
                block.is_visible()
            } else {
                true
            });
        out
    }
    pub fn check_collision_lb(&self, x: i32, y: i32, hit_box_y: i32, check_visible: bool) -> bool {
        let pos = self.get_block_id(x, y + hit_box_y);
        self.check_collision(pos, check_visible)
    }

    pub fn check_collision_lt(&self, x: i32, y: i32, check_visible: bool) -> bool {
        let pos = self.get_block_id(x, y);
        self.check_collision(pos, check_visible)
    }

    pub fn check_collision_lc(&self, x: i32, y: i32, hit_box_y: i32, check_visible: bool) -> bool {
        let pos = self.get_block_id(x, y + hit_box_y);
        self.check_collision(pos, check_visible)
    }

    pub fn check_collision_rc(
        &self,
        x: i32,
        y: i32,
        hit_box_x: i32,
        hit_box_y: i32,
        check_visible: bool,
    ) -> bool {
        let pos = self.get_block_id(x + hit_box_x, y + hit_box_y);
        self.check_collision(pos, check_visible)
    }

    pub fn check_collision_rb(
        &self,
        x: i32,
        y: i32,
        hit_box_x: i32,
        hit_box_y: i32,
        check_visible: bool,
    ) -> bool {
        let pos = self.get_block_id(x + hit_box_x, y + hit_box_y);
        self.check_collision(pos, check_visible)
    }

    pub fn check_collision_rt(&self, x: i32, y: i32, hit_box_x: i32, check_visible: bool) -> bool {
        let pos = self.get_block_id(x + hit_box_x, y);
        self.check_collision(pos, check_visible)
    }

    pub fn check_collision_with_platform(
        &self,
        x: i32,
        y: i32,
        hit_box_x: i32,
        hit_box_y: i32,
    ) -> i32 {
        for i in 0..self.platform.len() {
            let ref platform = self.platform[i];

            if -self.x_pos + x as f32 + hit_box_x as f32 >= platform.x_pos
                && -self.x_pos + x as f32 <= platform.x_pos + (platform.size * 16) as f32
            {
                let v = (y + hit_box_y) as f32;
                if v >= platform.y_pos && v <= platform.y_pos + 16 as f32 {
                    return i as i32;
                }
            }
        }
        return -1;
    }

    pub fn get_map_block(&self, x: i32, y: i32) -> &MapLevel {
        &self.map_of_level[x as usize][y as usize]
    }

    pub fn player_death(&self, animation: bool, instant_death: bool) {}

    pub fn block_use(&self, x: i32, y: i32, block_id: i32, pos: i32) {
        let level_map = &self.map_of_level[x as usize][y as usize];
        if pos == 0 {
            match block_id {
                8 | 55 => {
                    if level_map.spawn_mushroom {
                        if self.player.power_lvl == 0 {
                            self.minion[self.get_list_id(32 * x) as usize].push(value);
                        }
                    }
                }
                13 | 28 | 81 => {}
                24 => {}
                128 | 129 => {}
                _ => {}
            }
        }
    }
}
