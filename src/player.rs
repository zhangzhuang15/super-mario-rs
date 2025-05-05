use crate::cimg::CIMG;
use crate::core::Direction;
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
}
