mod block;
mod block_debris;
mod bubble;
mod cfg;
mod cimg;
mod coin;
mod core;
mod event;
mod flag;
mod game_manager;
mod level_text;
mod main_menu;
mod map;
mod map_level;
mod menu;
mod menu_manager;
mod menu_option;
mod minion;
mod pipe;
mod platform;
mod player;
mod points;
mod sprite;
mod text;

fn main() {
    let game_manager = game_manager::GameManager::new("uMario", cfg::GAME_WIDTH, cfg::GAME_HEIGHT);
}
