use std::collections::HashMap;

pub struct GameManager<'a> {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    context: sdl2::Sdl,
    texture_table: HashMap<&'static str, super::texture_context::TextureContext<'a>>,
}

impl<'a> GameManager<'a> {
    pub fn new(title: &str, width: u32, height: u32) -> GameManager {
        let context = sdl2::init().expect("fail to create sdl context");
        let video_system = context.video().expect("fail to create sdl video system");

        let mut window = video_system.window(title, width, height);
        window.position_centered();
        let window = window.build().expect("fail to create sdl window");

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .expect("fail to create sdl canvas render");

        // event_pump provides event for us
        let event_pump = context.event_pump().expect("fail to create sdl event pump");

        GameManager {
            canvas,
            event_pump,
            context,
            texture_table: HashMap::new(),
        }
    }
}
