#[derive(PartialEq, Clone, Copy)]
pub enum GameState {
    MainMenu,
    Playing,
    GameOver,
}

#[derive(PartialEq, Clone, Copy)]
pub enum MenuButton {
    NewGame,
    Continue,
    Options,
    Credits,
    Exit,
}
