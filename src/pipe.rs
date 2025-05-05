pub(crate) struct Pipe {
    // ----- 0 = VERTICAL, 1 = HORIZONTAL -> VERTICAL, 2 = VERTICAL -> VERTICAL
    i_type: i32,

    // ----- X, Y LEFT Block Position
    lx: i32,
    ly: i32,
    // ----- X, Y RIGHT Block Position
    rx: i32,
    ry: i32,

    new_player_pos_x: i32,
    new_player_pos_y: i32,

    // ----- MAP LVL ID
    new_current_level: i32,
    new_level_type: i32,
    new_move_map: bool,
    new_underwater: bool,
    delay: i32,
    speed: i32,
}
