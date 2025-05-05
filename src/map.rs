use crate::block::Block;
use crate::block_debris::BlockDebris;
use crate::bubble::Bubble;
use crate::coin::Coin;
use crate::event::Event;
use crate::flag::Flag;
use crate::level_text::LevelText;
use crate::map_level::MapLevel;
use crate::minion::Minion;
use crate::pipe::Pipe;
use crate::platform::Platform;
use crate::player::Player;
use crate::points::Points;

pub(crate) struct Map<'a> {
    x_pos: f32,
    y_pos: f32,

    block_size: i32,
    block: Vec<Block<'a>>,

    v_minion_size: i32,
    v_minion: Vec<Block<'a>>,

    map_width: i32,
    map_height: i32,
    map_of_level: Vec<Vec<MapLevel>>,

    block_debris: Vec<BlockDebris>,
    platform: Vec<Platform>,
    level_text: Vec<LevelText>,
    bubble: Vec<Bubble>,

    current_level_id: i32,
    level_type: i32,
    underwater: bool,
    spawn_point_id: i32,
    move_map: bool,
    frame_id: i32,
    map_time: i32,
    in_event: bool,
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

impl<'a> Map<'a> {
    pub fn new() -> Map<'a> {
        Map {
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
            player: Player::new(),
            flag: None,
            minion: Vec::new(),
            minion_size: 0,
            coin: Vec::new(),
            points: Vec::new(),
            pipe: Vec::new(),
        }
    }
}
