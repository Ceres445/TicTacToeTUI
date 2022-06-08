use core::fmt;

use adorn::adorn_method;
use tui::style::Color;

use crate::app::Score;

pub type Board = Vec<Vec<GameCell>>;

#[derive(Clone)]
pub enum GameState {
    GameOver(Option<Player>),
    GameInProgress(Board, Player, Position),
    Menu(u8)
}


pub type Position = (usize, usize);


pub struct Game {
    pub board: Board,
    pub current_position: Position,
    pub current_player: Player,
    pub winner: Option<Player>,
    pub warning_message: Option<String>,
    should_continue: bool,
    state_changed: bool,
}

#[derive(Copy, Clone)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub fn next(&self) -> Player {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }
    pub fn color(&self) -> Color {
        match self {
            Player::Player1 => Color::Red,
            Player::Player2 => Color::Blue,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::Player1 => write!(f, "Player 1 (X)"),
            Player::Player2 => write!(f, "Player 2 (O)"),
        }
    }
}

#[derive(Clone, PartialEq, Copy)]
pub enum GameCell {
    Empty,
    Cross,
    Circle,
}

impl GameCell {
    pub fn color(&self) -> Color {
        match self {
            GameCell::Empty => Color::White,
            GameCell::Cross => Color::Red,
            GameCell::Circle => Color::Blue,
        }
    }
    pub fn to_text(&self, pos: Option<(usize, usize)>) -> String {
        let centre = match self {
            GameCell::Empty => String::from("L"),
            GameCell::Cross => String::from("X"),
            GameCell::Circle => String::from("O"),
        };
        // TODO: Print proper positions with borders
        match pos {
            Some((x, y)) => match (x, y) {
                (0, 0) => format!("{}", centre),
                (0, 1) => format!("{}", centre),
                (0, 2) => format!("{}", centre),
                (1, 0) => format!("{}", centre),
                (1, 1) => format!("{}", centre),
                (1, 2) => format!("{}", centre),
                (2, 0) => format!("{}", centre),
                (2, 1) => format!("{}", centre),
                (2, 2) => format!("{}", centre),
                _ => panic!("Invalid coordinates: {}, {}", x, y),
            },
            None => centre,
        }
    }
}

impl Game {
    fn get_current_player_cell(&self) -> GameCell {
        match self.current_player {
            Player::Player1 => GameCell::Cross,
            Player::Player2 => GameCell::Circle,
        }
    }

    fn next(&mut self) {
        let check =
            |x: GameCell, y: GameCell, z: GameCell| x == y && y == z && x != GameCell::Empty;
        let mut rows = self.board.iter().map(|row| check(row[0], row[1], row[2]));
        let mut cols =
            (0..3).map(|col| check(self.board[0][col], self.board[1][col], self.board[2][col]));
        let diag1 = check(self.board[0][0], self.board[1][1], self.board[2][2]);
        let diag2 = check(self.board[0][2], self.board[1][1], self.board[2][0]);
        if rows.any(|x| x) || cols.any(|x| x) || diag1 || diag2 {
            self.winner = Some(self.current_player);
            self.should_continue = false;
        } else if self.board.iter().flatten().all(|x| *x != GameCell::Empty) {
            self.winner = None;
            self.should_continue = false;
        } else {
            self.current_player = self.current_player.next();
        }
    }
    pub fn is_over(&self) -> bool {
        self.should_continue == false
    }

    pub fn get_score(&self) -> Score {
        if self.winner.is_some() {
            match self.winner.unwrap() {
                Player::Player1 => Score {
                    player1: 1,
                    player2: 0,
                },
                Player::Player2 => Score {
                    player1: 0,
                    player2: 1,
                },
            }
        } else {
            Score::new()
        }
    }
    pub fn get_warning_message(&self) -> Option<String> {
        self.warning_message.clone()
    }
    pub fn place(&mut self) {
        self.state_changed = true;
        self.warning_message = None;
        if let Some(cell) = self
            .board
            .get(self.current_position.0)
            .and_then(|row| row.get(self.current_position.1))
        {
            match cell {
                GameCell::Empty => {
                    self.board[self.current_position.0][self.current_position.1] =
                        self.get_current_player_cell();
                    self.next();
                }
                _ => {
                    self.warning_message = Some("This cell is already taken!".to_string());
                    return;
                }
            }
        } else {
            println!("This cell is out of range!");
        }
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: vec![vec![GameCell::Empty; 3]; 3],
            current_position: (0, 0),
            current_player: Player::Player1,
            winner: None,
            should_continue: true,
            warning_message: None,
            state_changed: true,
        }
    }

    pub fn get_state(&mut self) -> Option<GameState> {
        if self.state_changed {
            self.state_changed = false;
            if self.is_over() {
                Some(GameState::GameOver(self.winner))
            } else {
                Some(GameState::GameInProgress(
                    self.board.clone(),
                    self.current_player,
                    self.current_position,
                ))
            }
        } else {
            None
        }
    }

    fn wrap<F>(&mut self, func: F)
    where
        F: Fn(&mut Self),
    {
        if self.should_continue {
            self.state_changed = true;
            func(self);
        } else {
            self.warning_message = Some("Game is over!".to_string());
        }
    }

    #[adorn_method(wrap)]
    pub fn on_up(&mut self) {
        self.warning_message = None;
        if self.current_position.0 > 0 {
            self.current_position.0 -= 1;
        }
    }

    #[adorn_method(wrap)]
    pub fn on_down(&mut self) {
        self.warning_message = None;
        if self.current_position.0 < 2 {
            self.current_position.0 += 1;
        }
    }

    #[adorn_method(wrap)]
    pub fn on_left(&mut self) {
        self.warning_message = None;
        if self.current_position.1 > 0 {
            self.current_position.1 -= 1;
        }
    }

    #[adorn_method(wrap)]
    pub fn on_right(&mut self) {
        self.warning_message = None;
        if self.current_position.1 < 2 {
            self.current_position.1 += 1;
        }
    }
}
