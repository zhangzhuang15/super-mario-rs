use sdl2::sys::{SDL_GetTicks, SDL_Renderer};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::cfg::GAME_HEIGHT;
use crate::cimg::CIMG;
use crate::core::Direction;
use crate::map::Map;
use crate::sprite::Sprite;

pub const MAX_MOVE_L: i32 = 4;
pub const SMALL_X: i32 = 24;
pub const SMALL_Y: i32 = 32;
pub const BIG_X: i32 = 32;
pub const BIG_Y: i32 = 64;

pub(crate) struct Player {
    mario: Vec<Sprite>,
    sprite_id: i32,
    move_animation_time: u32,
    mario_lvl_up: Option<CIMG>,

    x_pos: f32,
    y_pos: f32,

    num_of_lives: i32,

    unkillable: bool,
    star_effect: bool,
    unkillable_time_frame_id: i32,
    unkillable_frame_id: i32,

    in_level_down_animation: bool,
    in_level_down_animation_frame_id: i32,

    score: u32,
    coins: u32,

    combo_points: i32,
    frame_id: i32,

    power_lvl: i32,

    in_level_animation: bool,
    in_level_animation_type: bool, // -- true = UP, false = DOWN

    in_level_animation_frame_id: i32,

    // ----- LVL UP
    // ----- MOVE
    move_direction: Direction, // true = LEFT, false = RIGHT
    b_move: bool,
    change_move_direction: Direction,
    new_move_direction: Direction,

    current_max_move: i32,
    move_speed: i32,
    time_passed: u32,

    squat: bool,

    on_platform_id: i32,

    // ----- MOVE
    // ----- JUMP
    jump_state: i32,

    start_jump_speed: f32,
    current_jump_speed: f32,
    jump_distance: f32,
    current_jump_distance: f32,

    current_falling_speed: f32,

    spring_jump: bool,

    // ----- JUMP
    // ----- BUBBLE
    next_bubble_time: u32,
    next_fall_frame_id: i32,

    next_fireball_frame_id: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            mario: Vec::new(),
            sprite_id: 0,
            move_animation_time: 0,
            mario_lvl_up: None,
            x_pos: 0.0,
            y_pos: 0.0,
            num_of_lives: 0,
            unkillable: false,
            star_effect: false,
            unkillable_time_frame_id: 0,
            unkillable_frame_id: 0,
            in_level_down_animation: false,
            in_level_down_animation_frame_id: 0,
            score: 0,
            coins: 0,
            combo_points: 0,
            frame_id: 0,
            power_lvl: 0,
            in_level_animation: false,
            in_level_animation_type: false,
            in_level_animation_frame_id: 0,
            move_direction: Direction::Left,
            b_move: false,
            change_move_direction: Direction::Left,
            new_move_direction: Direction::Left,
            current_max_move: 0,
            move_speed: 0,
            time_passed: 0,
            squat: false,
            on_platform_id: 0,
            jump_state: 0,
            start_jump_speed: 0.0,
            current_jump_speed: 0.0,
            jump_distance: 0.0,
            current_jump_distance: 0.0,
            current_falling_speed: 0.0,
            spring_jump: false,
            next_bubble_time: 0,
            next_fall_frame_id: 0,
            next_fireball_frame_id: 0,
        }
    }

    pub fn from(render: *mut SDL_Renderer, x_pos: f32, y_pos: f32) -> Player {
        let mut player = Player::new();
        player.x_pos = x_pos;
        player.y_pos = y_pos;
        player.num_of_lives = 1000;
        player.sprite_id = 1;
        player.combo_points = 1;
        player.current_max_move = 4;
        player.on_platform_id = -1;
        player.start_jump_speed = 7.65;
        player.current_falling_speed = 2.7;
        player.next_fireball_frame_id = 8;

        player.time_passed = unsafe { SDL_GetTicks() };

        let mut temp_i: Vec<u32> = Vec::new();

        temp_i.push(0);
        // 小超级马力死亡
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_death".to_string()],
            temp_i.clone(),
            true,
        ));

        // 小超级马力站立
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario".to_string()],
            temp_i.clone(),
            true,
        ));

        // 小超级马力移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_move0".to_string()],
            temp_i.clone(),
            true,
        ));

        // 小超级马力移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_move1".to_string()],
            temp_i.clone(),
            true,
        ));

        // 小超级马力移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_move2".to_string()],
            temp_i.clone(),
            true,
        ));

        // 小超级马力跳起
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_jump".to_string()],
            temp_i.clone(),
            true,
        ));

        // 小超级马力刹车
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_st".to_string()],
            temp_i.clone(),
            true,
        ));

        // 小超级马力站立
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario".to_string()],
            temp_i.clone(),
            true,
        ));

        // 小超级马力水下移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_underwater0".to_string()],
            temp_i.clone(),
            true,
        ));

        // 小超级马力水下移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_underwater1".to_string()],
            temp_i.clone(),
            true,
        ));

        // 小超级马力水下停止
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_end".to_string()],
            temp_i.clone(),
            true,
        ));

        // 小超级马力水下停止
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_end1".to_string()],
            temp_i.clone(),
            true,
        ));

        // 大超级马力站立
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario1".to_string()],
            temp_i.clone(),
            true,
        ));

        // 大超级马力移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario1_move0".to_string()],
            temp_i.clone(),
            true,
        ));

        // 大超级马力移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario1_move1".to_string()],
            temp_i.clone(),
            true,
        ));

        // 大超级马力移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario1_move2".to_string()],
            temp_i.clone(),
            true,
        ));

        // 大超级马力跳起
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario1_jump".to_string()],
            temp_i.clone(),
            true,
        ));

        // 大超级马力刹车
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario1_st".to_string()],
            temp_i.clone(),
            true,
        ));

        // 大超级马力蹲下
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario1_squat".to_string()],
            temp_i.clone(),
            true,
        ));

        // 大超级马力水下移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario1_underwater0".to_string()],
            temp_i.clone(),
            true,
        ));

        // 大超级马力水下移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario1_underwater1".to_string()],
            temp_i.clone(),
            true,
        ));

        // 大超级马力水下停止
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario1_end".to_string()],
            temp_i.clone(),
            true,
        ));

        // 大超级马力水下停止
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario1_end1".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的大超级马力站立
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的大超级马力移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2_move0".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的大超级马力移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2_move1".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的大超级马力移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2_move2".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的大超级马力跳起
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2_jump".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的大超级马力刹车
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2_st".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的大超级马力蹲下
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2_squat".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的大超级马力水下移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2_underwater0".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的大超级马力水下移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2_underwater1".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的大超级马力水下停止
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2_end".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的大超级马力水下停止
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2_end1".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2s".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2s_move0".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2s_move1".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2s_move2".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2s_jump".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2s_st".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2s_squat".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2s_underwater0".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2s_underwater1".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2s_end".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario2s_end1".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s0".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s0_moved".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s0_move1".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s0_move2".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s0_jump".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s0_st".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s0".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s0_underwater0".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s0_underwater1".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s0_end".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s0_end1".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的小超级马力站立
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s1".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的小超级马力移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s1_move0".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的小超级马力移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s1_move1".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的小超级马力移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s1_move2".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的小超级马力跳起
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s1_jump".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的小超级马力刹车
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s1_st".to_string()],
            temp_i.clone(),
            true,
        ));

        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s1".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的小超级马力水下移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s1_underwater0".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的小超级马力水下移动
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s1_underwater1".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的小超级马力水下停止
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s1_end".to_string()],
            temp_i.clone(),
            true,
        ));

        // 会发波的小超级马力水下停止
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_s1_end1".to_string()],
            temp_i.clone(),
            true,
        ));

        // 小超级马力吃完蘑菇后，半长高的样子
        player.mario.push(Sprite::from(
            render,
            vec!["mario/mario_lvlup".to_string()],
            temp_i.clone(),
            true,
        ));

        player
    }

    pub fn hit_box_y(&self) -> i32 {
        if self.power_lvl == 0 {
            SMALL_Y
        } else if self.squat {
            44
        } else {
            BIG_Y
        }
    }

    pub fn hit_box_x(&self) -> i32 {
        if self.power_lvl == 0 { SMALL_X } else { BIG_X }
    }

    pub fn update_y_pos(&mut self, map: &mut Map, v: i32) {
        enum Status {
            NoChild,
            ChildFinished,
        }
        struct Context {
            val: i32,
            child_status: Status,
        }

        let arc_map = map;
        let mut contexts: Vec<Context> = Vec::new();
        contexts.push(Context {
            val: v,
            child_status: Status::NoChild,
        });

        loop {
            if contexts.len() == 0 {
                break;
            }

            let Context { val, child_status } = contexts.last().unwrap();

            match child_status {
                Status::ChildFinished => {
                    if (self.y_pos as i32) % 2 == 1 {
                        self.y_pos += 1.0;
                    }

                    if !arc_map.in_event
                        && self.y_pos - self.hit_box_y() as f32 > GAME_HEIGHT as f32
                    {
                        self.y_pos = -80.0;
                        arc_map.player_death(false, true);
                    }
                    contexts.pop();
                    if contexts.len() > 0 {
                        let context = contexts.last_mut().unwrap();
                        context.child_status = Status::ChildFinished;
                    }
                }
                Status::NoChild => {
                    let mut left = false;
                    let mut right = false;

                    if *val > 0 {
                        left = arc_map.check_collision_lb(
                            (self.x_pos - arc_map.x_pos + 2.0) as i32,
                            self.y_pos as i32 + val,
                            self.hit_box_y(),
                            true,
                        );
                        right = arc_map.check_collision_rb(
                            (self.x_pos - arc_map.x_pos - 2.0) as i32,
                            self.y_pos as i32 + val,
                            self.hit_box_x(),
                            self.hit_box_y(),
                            true,
                        );

                        if !left && !right {
                            self.y_pos += *val as f32;
                        } else {
                            if self.jump_state == 2 {
                                self.jump_state = 0;
                            }
                            contexts.push(Context {
                                val: val - 1,
                                child_status: Status::NoChild,
                            });
                            continue;
                        }
                    }

                    if *val < 0 {
                        left = arc_map.check_collision_lt(
                            (self.x_pos - arc_map.x_pos + 2.0) as i32,
                            self.y_pos as i32 + val,
                            false,
                        );
                        right = arc_map.check_collision_rt(
                            (self.x_pos - arc_map.x_pos - 2.0) as i32,
                            self.y_pos as i32 + val,
                            self.hit_box_x(),
                            false,
                        );

                        if arc_map.check_collision_with_platform(
                            self.x_pos as i32,
                            self.y_pos as i32,
                            0,
                            0,
                        ) >= 0
                            || arc_map.check_collision_with_platform(
                                self.x_pos as i32,
                                self.y_pos as i32,
                                self.hit_box_x(),
                                self.hit_box_y(),
                            ) >= 0
                        {
                            self.jump_state = 2;
                        } else if !left && !right {
                            self.y_pos += *val as f32;
                        } else {
                            if self.jump_state == 1 {}
                        }
                    }

                    if (self.y_pos as i32) % 2 == 1 {
                        self.y_pos += 1.0;
                    }

                    if !arc_map.in_event
                        && self.y_pos - self.hit_box_y() as f32 > GAME_HEIGHT as f32
                    {
                        self.y_pos = -80.0;
                        arc_map.player_death(false, true);
                    }

                    contexts.pop();
                    if contexts.len() > 0 {
                        let context = contexts.last_mut().unwrap();
                        context.child_status = Status::ChildFinished;
                    }
                }
            }
        }
    }

    pub fn player_physics(&mut self, map: &mut Map) {
        if !map.underwater {
            if self.jump_state == 1 {}
        }
    }
}
