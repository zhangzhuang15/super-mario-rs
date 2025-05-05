use crate::map::Map;
use sdl2::sys::SDL_Event;
use sdl2::sys::SDL_Renderer;
use sdl2::sys::SDL_Window;

const MIN_FRAME_TIME: i32 = 16;

static mut MOVE_PRESSED: bool = false;
static mut KEY_MENU_PRESSED: bool = false;
static mut KEY_S: bool = false;
static mut KEY_W: bool = false;
static mut KEY_A: bool = false;
static mut KEY_D: bool = false;
static mut KEY_SHIFT: bool = false;
static mut KEY_A_PRESSED: bool = false;
static mut KEY_D_PRESSED: bool = false;
static mut QUIT_GAME: bool = false;
static mut MOUSE_LEFT_PRESSED: bool = false;
static mut MOUSE_RIGHT_PRESSED: bool = false;
static mut MOUSE_X: i32 = 0;
static mut MOUSE_Y: i32 = 0;

pub enum Direction {
    Left,
    Right,
}

pub(crate) struct Core<'a> {
    pub window: SDL_Window,
    pub renderer: SDL_Renderer,
    pub main_event: SDL_Event,
    pub frame_time: i64,
    pub l_fps_time: u64,
    pub inum_of_fps: i32,
    pub ifps: i32,
    pub first_dir: Direction,
    pub o_map: Map<'a>,
}
