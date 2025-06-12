use crate::{
    cfg::{GAME_HEIGHT, GAME_WIDTH},
    map::Map,
};
use sdl2::sys::{
    SDL_CreateRenderer, SDL_CreateWindow, SDL_Delay, SDL_Event, SDL_EventType, SDL_GetTicks,
    SDL_PollEvent, SDL_RenderClear, SDL_RenderFillRect, SDL_RenderPresent, SDL_Renderer,
    SDL_RendererFlags, SDL_WINDOWPOS_CENTERED_MASK, SDL_Window, SDL_WindowFlags,
};
use std::{
    ptr::null,
    sync::{Arc, Mutex},
};

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

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

pub(crate) struct Core {
    pub window: SDL_Window,
    pub renderer: SDL_Renderer,
    pub main_event: SDL_Event,
    pub frame_time: i64,
    pub fps_time: u64,
    pub num_of_fps: i32,
    pub fps: i32,
    pub first_dir: Direction,
    pub map: Arc<Mutex<Map>>,
}

impl Core {
    pub fn new() -> Core {
        let mut window = unsafe {
            SDL_CreateWindow(
                "uMario - www.LukaszJakowski.pl".as_ptr() as *const i8,
                SDL_WINDOWPOS_CENTERED_MASK as i32,
                SDL_WINDOWPOS_CENTERED_MASK as i32,
                GAME_WIDTH as i32,
                GAME_HEIGHT as i32,
                SDL_WindowFlags::SDL_WINDOW_SHOWN as u32,
            )
        };
        if window.is_null() {
            QUIT_GAME = true;
        }

        let mut render = unsafe {
            SDL_CreateRenderer(
                window,
                -1,
                SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32,
            )
        };

        let main_event = SDL_Event {};
    }
    pub fn resolve_input(&self) {}

    pub fn resolve_mouse_input(&self) {}

    pub fn update(&self) {}

    pub fn draw(&self) {}

    pub fn main_loop(&mut self) {
        self.fps_time = unsafe { SDL_GetTicks().into() };

        loop {
            unsafe {
                if QUIT_GAME {
                    break;
                }

                if self.main_event.type_ == SDL_EventType::SDL_QUIT as u32 {
                    break;
                }
            }

            self.frame_time = unsafe { SDL_GetTicks().into() };

            unsafe {
                SDL_PollEvent(&mut self.main_event as *mut SDL_Event);
                SDL_RenderClear(&mut self.renderer as *mut SDL_Renderer);

                SDL_RenderFillRect(&mut self.renderer as *mut SDL_Renderer, null());
            }

            self.resolve_input();
            self.resolve_mouse_input();
            self.update();
            self.draw();

            unsafe {
                SDL_RenderPresent(&mut self.renderer as *mut SDL_Renderer);
                let ticks = SDL_GetTicks();
                let distance = ticks - self.frame_time as u32;
                let limit = MIN_FRAME_TIME as u32;
                if distance < limit {
                    SDL_Delay(limit - distance);
                }
            };
        }
    }
}
