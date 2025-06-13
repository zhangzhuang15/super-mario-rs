mod cfg;
mod cimg;
mod core;
mod game_manager;
mod main_menu;
mod map;
mod player;
mod sprite;
mod texture_context;

fn main() {
    let game_manager = game_manager::GameManager::new(
        "uMario - www.LukaszJakowski.pl",
        cfg::GAME_WIDTH,
        cfg::GAME_HEIGHT,
    );
}
