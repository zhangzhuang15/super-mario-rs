pub enum PlatformType {
    Bot,
    Top,
    BotAndTop,
    LeftAndRight,
    Bonus,
    Falling,
    SeeSaw,
    FallingSeeSaw,
}

pub enum PlatformDirection {
    RightOrTop,
    LeftOrBot,
}

pub(crate) struct Platform {
    i_type: PlatformType,
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,

    x_pos: f32,
    y_pos: f32,

    direction: PlatformDirection,

    size: i32,
    on: bool,

    seesaw_platform_id: i32,
}

impl Platform {
    pub fn new() -> Platform {
        Platform {
            i_type: PlatformType::Top,
            x_start: 0,
            x_end: 0,
            y_start: 0,
            y_end: 0,
            x_pos: 0.0,
            y_pos: 0.0,
            direction: PlatformDirection::LeftOrBot,
            size: 0,
            on: false,
            seesaw_platform_id: 0,
        }
    }
}
