use crate::{
    app::App,
    game::{Board, GameState, Player, Position},
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(f.size());
    let main = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .direction(Direction::Horizontal)
        .split(rects[0]);
    let state = &app.state;
    match state {
        GameState::GameInProgress(board, _, pos) => {
            draw_board(f, board.to_vec(), *pos, &main[0]);
        }
        GameState::GameOver(winner) => {
            draw_game_over(f, &rects[0], *winner);
        }
        GameState::Menu(row) => {
            draw_menu(f, &rects[0], *row);
        }
    }
    match &app.warning_message {
        Some(message) => draw_warning(f, &rects[1], message.to_string()),
        None => draw_info(f, &rects[1], state),
    }
    draw_score(f, app, &main[1]);
}

fn draw_menu<B: Backend>(f: &mut Frame<B>, rect: &Rect, row: u8) {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let table = Table::new([
        Row::new([Cell::from("Resume Game")]),
        Row::new([Cell::from("New Game")]),
        Row::new([Cell::from("Quit")]),
    ])
    .block(Block::default().borders(Borders::ALL).title("Menu"))
    .widths(&[Constraint::Percentage(100)])
    .highlight_style(selected_style)
    .highlight_symbol(">>");

    let mut state = TableState::default();
    state.select(Some(row.into()));
    f.render_stateful_widget(table, *rect, &mut state)
}

pub fn draw_score<B: Backend>(f: &mut Frame<B>, app: &mut App, rect: &Rect) {
    let table = Table::new(vec![
        Row::new(vec![Cell::from("Score:".to_string())]),
        Row::new(vec![Cell::from(format!(
            "Player 1's score: {}\nPlayer 2's score: {}",
            app.score.player1, app.score.player2
        ))])
        .style(Style::default().fg(Color::Yellow))
        .height(2),
        if let GameState::GameInProgress(_, player, _) = app.state {
            Row::new(vec![Cell::from(format!("{}'s turn", player,))])
                .style(Style::default().fg(player.color()))
        } else {
            Row::new(vec![Cell::from("Game Over".to_string())])
                .style(Style::default().fg(Color::Red))
        },
    ])
    .block(Block::default().borders(Borders::ALL))
    .widths(&[Constraint::Percentage(100)]);

    f.render_widget(table, *rect)
}

pub fn draw_game_over<B: Backend>(f: &mut Frame<B>, rect: &Rect, winner: Option<Player>) {
    let winning_message = match winner {
        Some(winner) => format!("{} wins!", winner),
        None => "It's a draw!".to_string(),
    };
    let block = Paragraph::new(format!("Game over! \n{}", winning_message))
        .block(Block::default().title("Game Over").borders(Borders::ALL))
        .style(
            Style::default()
                .fg(if let Some(player) = winner {
                    player.color()
                } else {
                    Color::Gray
                })
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(block, *rect);
}

pub fn draw_warning<B: Backend>(f: &mut Frame<B>, rect: &Rect, message: String) {
    let block = Paragraph::new(message)
        .block(Block::default().title("Warning").borders(Borders::ALL))
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
    f.render_widget(block, *rect);
}

pub fn draw_board<B: Backend>(f: &mut Frame<B>, board: Board, pos: Position, rect: &Rect) {
    // TODO: Make it look like a tic tac toe board
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let rows = board.iter().enumerate().map(|(i, item)| {
        // let height = item
        //     .iter()
        //     .map(|content| content.to_text(None).chars().filter(|c| *c == '\n').count())
        //     .max()
        //     .unwrap_or(0)
        //     + 1;
        let cells = item.iter().enumerate().map(|(j, c)| {
            Cell::from(Span::raw(c.to_text(Some((i, j))))).style(if (i, j) == pos {
                selected_style
            } else {
                Style::default().fg(c.color())
            })
        });
        Row::new(cells).height(rect.height / 3)
    });
    let t = Table::new(rows)
        .block(Block::default().borders(Borders::ALL))
        .widths(&[Constraint::Ratio(1, 3); 3]);
    f.render_widget(t, *rect)
}

pub fn draw_info<B: Backend>(f: &mut Frame<B>, rect: &Rect, state: &GameState) {
    let info = match state {
        GameState::GameInProgress(_, _, _) => "Game in progress...\nPress M/ Esc to open the Game Menu\nPress P to place a piece, Q to \
            quit, or R to reset the board.\nUse the arrow keys to move the piece.".to_string(),
        GameState::GameOver(_) => "Game over!\nPress M/ Esc to open the Game Menu\nPress R to reset the board or Q to quit."
            .to_string(),
        GameState::Menu(_) => "Tic Tac Toe Menu\nPress Q to quit, or use the up and down arrow keys to select an item."
            .to_string(),
    };
    let text_block =
        Paragraph::new(info).block(Block::default().title("Info").borders(Borders::ALL));
    f.render_widget(text_block, *rect);
}
