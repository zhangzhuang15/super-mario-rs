use sdl2::sys::{SDL_GetTicks, SDL_Renderer};

use crate::cimg::CIMG;
pub(crate) struct Sprite {
    sprite: Vec<CIMG>,
    delay: Vec<u32>,
    rotate: bool,
    current_frame: i32,
    start_frame: i32,
    end_frame: i32,
    time_passed: u64,
}

impl Sprite {
    pub fn new() -> Sprite {
        Sprite {
            sprite: Vec::new(),
            delay: Vec::new(),
            rotate: false,
            current_frame: 0,
            start_frame: 0,
            end_frame: 0,
            time_passed: 0,
        }
    }

    pub fn from(
        render: &mut SDL_Renderer,
        sprite: Vec<String>,
        delay: Vec<u32>,
        rotate: bool,
    ) -> Sprite {
        let mut s = Sprite::new();
        s.delay = delay;
        s.rotate = rotate;
        s.end_frame = (sprite.len() as i32) - 1;

        sprite.into_iter().for_each(|ss| {
            s.sprite.push(CIMG::from(ss, render));
        });

        s
    }

    pub fn update(&mut self) {
        let current = unsafe { SDL_GetTicks() };
        let current_frame = self.current_frame as u32;
        let time_passed = self.time_passed as u32;
        let end_frame = self.end_frame as u32;

        if current - self.delay[current_frame as usize] > time_passed {
            self.time_passed = unsafe { SDL_GetTicks() as u64 };

            if current_frame == end_frame {
                self.current_frame = 0;
            } else {
                self.current_frame += 1;
            }
        }
    }

    pub fn get_current_texture(&mut self) -> &mut CIMG {
        &mut self.sprite[self.current_frame as usize]
    }

    pub fn get_texture_by_id(&mut self, id: i32) -> &mut CIMG {
        &mut self.sprite[id as usize]
    }
}
