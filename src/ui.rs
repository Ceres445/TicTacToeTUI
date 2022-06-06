use crate::app::Game;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut Game) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(f.size());
    if app.should_continue {
        draw_board(f, app, &rects[0]);
    } else {
        draw_game_over(f, &rects[0], app);
    }
    match &app.warning_message {
        Some(message) => draw_warning(f, &rects[1], message.to_string()),
        None => draw_info(f, &rects[1], app.should_continue),
    }
}

pub fn draw_game_over<B: Backend>(f: &mut Frame<B>, rect: &Rect, app: &Game) {
    let winning_message = match app.winner {
        Some(winner) => format!("{} wins!", winner),
        None => "It's a draw!".to_string(),
    };
    let block = Paragraph::new(format!("Game over! \n{}", winning_message))
        .block(Block::default().title("Game Over").borders(Borders::ALL))
        .style(
            Style::default()
                .fg(if let Some(_) = app.winner {
                    Color::Green
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

pub fn draw_board<B: Backend>(f: &mut Frame<B>, app: &mut Game, rect: &Rect) {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let rows = app.board.iter().enumerate().map(|(i, item)| {
        let height = item
            .iter()
            .map(|content| content.to_text(None).chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().enumerate().map(|(j, c)| {
            Cell::from(c.to_text(Some((i, j)))).style(if (i, j) == app.current_position {
                selected_style
            } else {
                Style::default()
            })
        });
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .block(Block::default().borders(Borders::ALL))
        .widths(&[Constraint::Ratio(1, 3); 3]);
    f.render_widget(t, *rect)
}

pub fn draw_info<B: Backend>(f: &mut Frame<B>, rect: &Rect, game_is_running: bool) {
    let info = String::from(if game_is_running {
        "Press P to place a piece, Q to quit, or R to reset the board.\nUse the arrow keys to move the piece."
    } else {
        "Press R to reset the board or Q to quit."
    });
    let text_block =
        Paragraph::new(info).block(Block::default().title("Info").borders(Borders::ALL));
    f.render_widget(text_block, *rect);
}
