use crate::game::{Game, GameState};
use std::ops::AddAssign;

use crossterm::event::{KeyCode, KeyEvent};

pub struct App {
    pub name: String,
    game: Game,
    pub score: Score,
    pub quit: bool,
    pub state: GameState,
    pub warning_message: Option<String>,
    pub prev_state: Option<GameState>,
}

pub struct Score {
    pub player1: u32,
    pub player2: u32,
}

impl Score {
    pub fn new() -> Score {
        Score {
            player1: 0,
            player2: 0,
        }
    }
}

impl AddAssign for Score {
    fn add_assign(&mut self, other: Score) -> () {
        self.player1 += other.player1;
        self.player2 += other.player2;
    }
}

impl App {
    pub fn new(name: String) -> App {
        let mut game = Game::new();
        let state = game.get_state().unwrap();
        App {
            name,
            game,
            score: Score::new(),
            quit: false,
            state,
            warning_message: None,
            prev_state: None,
        }
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }

    fn show_menu(&mut self) {
        if let GameState::Menu(_) = self.state {
            self.state = self.prev_state.clone().unwrap();
        } else {
            self.prev_state = Some(self.state.clone());
            self.state = GameState::Menu(0);
        }
    }
    fn next_row_menu(&mut self, up: bool) {
        if let GameState::Menu(i) = self.state {
            if up {
                if i < 2 {
                    self.state = GameState::Menu(i + 1);
                } else {
                    self.state = GameState::Menu(0);
                }
            } else {
                if i > 0 {
                    self.state = GameState::Menu(i - 1);
                } else {
                    self.state = GameState::Menu(2);
                }
            }
        }
    }

    pub fn update(&mut self, key: KeyEvent) {
        match self.state {
            GameState::Menu(x) => match key.code {
                KeyCode::Char(c) => match c {
                    'q' => self.quit(),
                    'm' => self.show_menu(),
                    _ => {}
                },
                KeyCode::Enter => match x {
                    0 => self.show_menu(),
                    1 => self.reset(),
                    2 => self.quit(),
                    _ => self.state = GameState::Menu(0),
                },
                KeyCode::Up => self.next_row_menu(false),
                KeyCode::Down => self.next_row_menu(true),
                _ => {}
            },
            _ => match key.code {
                KeyCode::Char(c) => self.on_key(c),
                KeyCode::Esc => self.show_menu(),
                KeyCode::Up => self.game.on_up(),
                KeyCode::Down => self.game.on_down(),
                KeyCode::Left => self.game.on_left(),
                KeyCode::Right => self.game.on_right(),
                _ => {}
            },
        };
        if let Some(state) = self.game.get_state() {
            self.state = state;
            match self.state {
                GameState::GameOver(_) => self.score += self.game.get_score(),
                _ => {}
            }
            self.warning_message = self.game.get_warning_message();
        }
    }
    fn reset(&mut self) {
        self.game = Game::new();
        self.state = self.game.get_state().unwrap();
        self.warning_message = self.game.get_warning_message();
    }

    pub fn on_key(&mut self, char: char) {
        match char {
            'p' => {
                if !self.game.is_over() {
                    self.game.place()
                }
            }
            'q' => self.quit(),
            'r' => self.reset(),
            'm' => self.show_menu(),
            _ => {
                if self.game.is_over() {
                    self.game.warning_message = None;
                }
            }
        };
    }
}
