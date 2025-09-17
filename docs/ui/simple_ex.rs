use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};
use std::{io::stdout, time::Duration};
use unicode_width::UnicodeWidthStr;

fn main() -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();
    let mut messages: Vec<String> = Vec::new();
    let mut scroll: usize = 0;

    // std::thread::sleep(std::time::Duration::from_secs(3));

    loop {
        terminal.draw(|f| {
            let size = f.size();
            // top: logs, divider line, bottom: single-line input
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Min(3),
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(size);

            // messages area (no box)
            let items: Vec<ListItem> = messages
                .iter()
                .map(|m| ListItem::new(Spans::from(Span::raw(m))))
                .collect();
            let messages_list = List::new(items).style(Style::default());
            f.render_widget(messages_list, chunks[0]);

            // divider line (single horizontal line)
            let divider = Paragraph::new(Span::styled(
                "─".repeat(chunks[1].width as usize),
                Style::default().add_modifier(Modifier::DIM),
            ));
            f.render_widget(divider, chunks[1]);

            // input rendered as plain text on its line (no box)
            let input_para = Paragraph::new(input.as_ref()).wrap(Wrap { trim: false });
            f.render_widget(input_para, chunks[2]);

            // cursor inside input line (account for unicode width)
            let x = chunks[2].x + UnicodeWidthStr::width(input.as_str()) as u16;
            let y = chunks[2].y;
            f.set_cursor(x, y);
        })?;

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code, modifiers, ..
                }) => match code {
                    KeyCode::Char('c') if modifiers == KeyModifiers::CONTROL => break,
                    KeyCode::Esc => break,
                    KeyCode::Enter => {
                        if !input.trim().is_empty() {
                            messages.push(input.drain(..).collect());
                        } else {
                            input.clear();
                        }
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Char(ch) => {
                        input.push(ch);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
