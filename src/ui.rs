use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, AppState};

pub fn draw(f: &mut Frame, app: &App) {
    match app.state {
        AppState::SelectDictionary => draw_select_dictionary(f, app),
        AppState::Training => draw_training(f, app),
        AppState::Quit => {}
    }
}

fn draw_select_dictionary(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Header
    let header = Paragraph::new("Foo Fight - Speed Typing Trainer")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Dictionary list
    let items: Vec<ListItem> = app
        .dictionaries
        .iter()
        .enumerate()
        .map(|(i, dict_meta)| {
            let style = if i == app.selected_dict_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let prefix = if i == app.selected_dict_index {
                "► "
            } else {
                "  "
            };
            ListItem::new(format!("{}{}", prefix, dict_meta.name)).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Select Dictionary"),
    );
    f.render_widget(list, chunks[1]);

    // Footer
    let footer = Paragraph::new("↑↓: Navigate | Enter: Select | Esc/q: Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn draw_training(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Header with dictionary name and stats
    let header_text = if let (Some(dict), Some(session)) = (&app.current_dictionary, &app.session) {
        format!(
            "{} | Progress: {}/{} | Correct: {} | Streak: {} | Success: {:.1}%",
            dict.name,
            session.current_index + 1,
            session.total_items(),
            session.correct_count,
            session.streak,
            session.success_rate()
        )
    } else {
        "Training".to_string()
    };

    let header = Paragraph::new(header_text)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Prompt
    let prompt_text = app.get_current_prompt().unwrap_or("No prompt");
    let prompt = Paragraph::new(prompt_text)
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
        .block(Block::default().borders(Borders::ALL).title("Question"));
    f.render_widget(prompt, chunks[1]);

    // Input field
    let input_style = if app.show_feedback {
        if app.is_correct {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Red)
        }
    } else {
        Style::default().fg(Color::White)
    };

    let input = Paragraph::new(app.input.as_str())
        .style(input_style)
        .block(Block::default().borders(Borders::ALL).title("Your Answer"));
    f.render_widget(input, chunks[2]);

    // Feedback area
    if app.show_feedback {
        let feedback_style = if app.is_correct {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Red)
        };

        let feedback_text = if app.is_correct {
            vec![
                Line::from(Span::styled(&app.feedback_message, feedback_style)),
                Line::from(""),
                Line::from(Span::styled(
                    "Press Enter to continue",
                    Style::default().fg(Color::Gray),
                )),
            ]
        } else {
            vec![
                Line::from(Span::styled(&app.feedback_message, feedback_style)),
                Line::from(""),
                Line::from(Span::styled(
                    "[R]etry | [S]kip",
                    Style::default().fg(Color::Yellow),
                )),
            ]
        };

        let feedback = Paragraph::new(feedback_text)
            .block(Block::default().borders(Borders::ALL).title("Feedback"))
            .wrap(Wrap { trim: false });
        f.render_widget(feedback, chunks[3]);
    } else {
        let help = Paragraph::new("Type the command and press Enter to validate")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Help"));
        f.render_widget(help, chunks[3]);
    }

    // Footer
    let footer = Paragraph::new("Ctrl+C/Esc: Back to menu | Enter: Submit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[4]);
}
