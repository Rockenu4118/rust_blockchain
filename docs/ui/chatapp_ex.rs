#![allow(warnings)]

use anyhow::Result;
use chrono::Local;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{List, ListItem, Paragraph, Wrap},
};
use std::{io::stdout, time::Duration};
use unicode_width::UnicodeWidthStr;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();
    let mut messages: Vec<(String, String)> = Vec::new(); // (timestamp, text)

    loop {
        terminal.draw(|f| {
            let size = f.size();

            // vertical layout: messages area, status line, input line
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

            // Render messages like a chat: timestamp dim, message normal
            let items: Vec<ListItem> = messages
                .iter()
                .map(|(ts, text)| {
                    let mut spans = Vec::new();
                    spans.push(Span::styled(
                        format!("[{}] ", ts),
                        Style::default().fg(Color::DarkGray),
                    ));
                    // very simple nick-like highlight example: bold first word if present
                    if let Some((first, rest)) = text.split_once(' ') {
                        spans.push(Span::styled(
                            format!("{} ", first),
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ));
                        spans.push(Span::raw(rest));
                    } else {
                        spans.push(Span::raw(text));
                    }
                    ListItem::new(Spans::from(spans))
                })
                .collect();

            let messages_list = List::new(items);
            f.render_widget(messages_list, chunks[0]);

            // status line: channel/name on left, hints on right
            let status_left = Span::styled(
                " #general ",
                Style::default()
                    .bg(Color::Blue)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            );
            let status_right = Span::styled(
                " Esc:quit  Enter:send ",
                Style::default().fg(Color::DarkGray),
            );
            let status = Paragraph::new(Spans::from(vec![
                status_left,
                Span::raw(" "),
                Span::raw(""),
            ]))
            .alignment(Alignment::Left);
            // draw status background by rendering a full-width styled line
            let status_bg = Paragraph::new(Span::styled(
                format!("{:width$}", "", width = chunks[1].width as usize),
                Style::default().bg(Color::Blue),
            ));
            f.render_widget(status_bg, chunks[1]);
            // overlay status text
            f.render_widget(status, chunks[1]);
            // render right-side hints (manually place a small paragraph)
            let hint = Paragraph::new(status_right).alignment(Alignment::Right);
            f.render_widget(hint, chunks[1]);

            // input line: leading prompt like "yournick> "
            let prompt = "you> ";
            let input_para =
                Paragraph::new(format!("{}{}", prompt, input)).wrap(Wrap { trim: false });

            f.render_widget(input_para, chunks[2]);

            // set cursor after prompt + input (account for unicode)
            let x =
                chunks[2].x + prompt.len() as u16 + UnicodeWidthStr::width(input.as_str()) as u16;
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
                            let ts = Local::now().format("%H:%M:%S").to_string();
                            let text = input.drain(..).collect();
                            messages.push((ts, text));
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

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
