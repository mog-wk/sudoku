// functions for clickbox event propriety

use sudoku::AppState;

pub fn quit() -> AppState {
    println!("Exitting - 0");

    std::process::exit(0);
    AppState::Quit
}

pub fn about() -> AppState {
    AppState::About
}

pub fn settings() -> AppState {
    AppState::Settings
}
pub fn game() -> AppState {
    //app_state = AppState::InGame;
    AppState::InGame
}
