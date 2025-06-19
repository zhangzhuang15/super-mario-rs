use std::collections::HashMap;

use sdl2::{
    image::LoadSurface,
    mixer::{InitFlag, open_audio},
    surface::Surface,
};

use crate::music_manager;

use super::music_manager::MusicManager;

pub struct GameManager<'a> {
    /// UI painter
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    /// Game event generator
    event_pump: sdl2::EventPump,
    context: sdl2::Sdl,
    /// Save textures that are painted in window conditionally, such as Mario, Coin, Mushroom
    texture_table: HashMap<&'static str, super::texture_context::TextureContext<'a>>,
    music_manager: MusicManager,
}

impl<'a> GameManager<'a> {
    pub fn new(title: &str, width: u32, height: u32) -> GameManager {
        let context = sdl2::init().expect("fail to create sdl context");
        let video_system = context.video().expect("fail to create sdl video system");

        let mut window = video_system.window(title, width, height);
        window.position_centered();

        let mut window = window.build().expect("fail to create sdl window");

        window.set_icon(
            Surface::from_file("files/images/ico.bmp").expect("failed to create window icon"),
        );

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .expect("fail to create sdl canvas render");

        // event_pump provides event for us
        let event_pump = context.event_pump().expect("fail to create sdl event pump");

        let music_manager = MusicManager::new();

        GameManager {
            canvas,
            event_pump,
            context,
            texture_table: HashMap::new(),
            music_manager,
        }
    }
}
