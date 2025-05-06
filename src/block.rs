use crate::map::Map;
use crate::sprite::Sprite;
use sdl2::sys::SDL_Renderer;
use std::sync::Arc;

pub(crate) struct Block {
    block_id: i32,
    collision: bool,
    death: bool,
    using: bool,
    visible: bool,
    pub sprite: Box<Sprite>,
}

impl Block {
    pub fn new() -> Block {
        Block {
            block_id: 0,
            collision: false,
            death: false,
            using: false,
            visible: true,
            sprite: Box::new(Sprite::new()),
        }
    }

    pub fn draw(&mut self, render: *mut SDL_Renderer, x_offset: i32, y_offset: i32) {
        self.sprite
            .get_current_texture()
            .draw(render, x_offset, y_offset, false);
    }

    pub fn is_collision(&self) -> bool {
        self.collision
    }

    pub fn is_death(&self) -> bool {
        self.death
    }

    pub fn is_using(&self) -> bool {
        self.using
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn update(&mut self) {}
}
