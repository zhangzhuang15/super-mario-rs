use crate::core::Direction;

pub(crate) struct MapLevel {
    pub block_id: i32,

    pub spawn_mushroom: bool,
    power_up: bool, // -- true = powerUP, false = 1UP
    spawn_star: bool,
    num_of_use: i32,

    // ----- Animation -----
    block_animation: bool,
    y_pos: i32,
    y_direction: Direction, // ----- true TOP, false BOTTOM

                            // ----- Animation -----
}
