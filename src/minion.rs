use crate::core::Direction;

pub(crate) struct Minion {
    minion_state: i32,

    kill_other_units: bool,

    block_id: i32,
    x_pos: f32,
    y_pos: f32,
    hit_box_x: i32,
    hit_box_y: i32,
    minion_spawned: bool,
    collision_only_with_player: bool,
    dead_time: i32,

    on_another_minion: bool,

    // ----- true = LEFT, false = RIGHT
    move_direction: Direction,
    move_speed: i32,

    jump_state: i32,

    start_jump_speed: f32,
    current_jump_speed: f32,
    jump_distance: f32,
    current_jump_distance: f32,
    current_falling_speed: f32,
}
