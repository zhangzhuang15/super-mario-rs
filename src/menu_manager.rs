use crate::cimg::CIMG;

pub enum GameState {
    MainMenu,
    GameLoading,
    Game,
    About,
    Options,
    Pasue,
}

pub(crate) struct MenuManager {
    pub active_option: CIMG,
    pub main_menu: MainMenu,
    pub loading_menu: LoadingMenu,
    pub about_menu: AboutMenu,
    pub options_menu: OptionsMenu,
    pub pause_menu: PauseMenu,
    pub current_game_state: GameState,
}
