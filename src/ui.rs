use crate::dictionary::Dictionary;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    SelectDictionary,
    Training,
    Results,
}

#[derive(Debug)]
pub struct Stats {
    pub correct: usize,
    pub incorrect: usize,
    pub total: usize,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            correct: 0,
            incorrect: 0,
            total: 0,
        }
    }

    pub fn accuracy(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.correct as f64 / self.total as f64) * 100.0
        }
    }
}

pub struct App {
    pub state: AppState,
    pub dictionaries: Vec<(PathBuf, Dictionary)>,
    pub selected_dict_index: usize,
    pub current_dict: Option<Dictionary>,
    pub current_item_index: usize,
    pub input: String,
    pub feedback: Option<(bool, String)>,
    pub stats: Stats,
    pub should_quit: bool,
}

impl App {
    pub fn new(dictionaries: Vec<(PathBuf, Dictionary)>) -> Self {
        Self {
            state: AppState::SelectDictionary,
            dictionaries,
            selected_dict_index: 0,
            current_dict: None,
            current_item_index: 0,
            input: String::new(),
            feedback: None,
            stats: Stats::new(),
            should_quit: false,
        }
    }

    pub fn select_dictionary(&mut self) {
        if let Some((_, dict)) = self.dictionaries.get(self.selected_dict_index) {
            self.current_dict = Some(dict.clone());
            self.current_item_index = 0;
            self.stats = Stats::new();
            self.state = AppState::Training;
        }
    }

    pub fn next_dict(&mut self) {
        if !self.dictionaries.is_empty() {
            self.selected_dict_index = (self.selected_dict_index + 1) % self.dictionaries.len();
        }
    }

    pub fn prev_dict(&mut self) {
        if !self.dictionaries.is_empty() {
            if self.selected_dict_index == 0 {
                self.selected_dict_index = self.dictionaries.len() - 1;
            } else {
                self.selected_dict_index -= 1;
            }
        }
    }

    pub fn submit_answer(&mut self) {
        if let Some(dict) = &self.current_dict {
            if let Some(item) = dict.items.get(self.current_item_index) {
                let is_correct = item.is_correct(&self.input);
                self.stats.total += 1;
                
                if is_correct {
                    self.stats.correct += 1;
                    self.feedback = Some((true, "Correct!".to_string()));
                } else {
                    self.stats.incorrect += 1;
                    self.feedback = Some((false, format!("Wrong! Expected: {}", item.command)));
                }
                
                self.input.clear();
            }
        }
    }

    pub fn next_question(&mut self) {
        self.feedback = None;
        if let Some(dict) = &self.current_dict {
            self.current_item_index += 1;
            if self.current_item_index >= dict.items.len() {
                self.state = AppState::Results;
            }
        }
    }

    pub fn back_to_menu(&mut self) {
        self.state = AppState::SelectDictionary;
        self.current_dict = None;
        self.current_item_index = 0;
        self.input.clear();
        self.feedback = None;
        self.stats = Stats::new();
    }
}

pub fn render(f: &mut Frame, app: &App) {
    match app.state {
        AppState::SelectDictionary => render_dictionary_selection(f, app),
        AppState::Training => render_training(f, app),
        AppState::Results => render_results(f, app),
    }
}

fn render_dictionary_selection(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title = Paragraph::new("Foo Fight - Speed Typing Training")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    if app.dictionaries.is_empty() {
        let msg = Paragraph::new("No dictionaries found in dictionaries/ folder")
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Dictionaries"));
        f.render_widget(msg, chunks[1]);
    } else {
        let items: Vec<ListItem> = app
            .dictionaries
            .iter()
            .enumerate()
            .map(|(i, (_, dict))| {
                let style = if i == app.selected_dict_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                
                let content = vec![
                    Line::from(vec![
                        Span::styled(&dict.name, style),
                    ]),
                    Line::from(vec![
                        Span::styled(format!("  {} items", dict.items.len()), Style::default().fg(Color::Gray)),
                    ]),
                ];
                ListItem::new(content)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Select Dictionary"));
        f.render_widget(list, chunks[1]);
    }

    let help = Paragraph::new("↑/↓: Navigate | Enter: Select | q: Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);
}

fn render_training(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    if let Some(dict) = &app.current_dict {
        let title = Paragraph::new(format!("{} - Question {}/{}", 
            dict.name, 
            app.current_item_index + 1,
            dict.items.len()
        ))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        let stats_text = format!(
            "Correct: {} | Incorrect: {} | Accuracy: {:.1}%",
            app.stats.correct,
            app.stats.incorrect,
            app.stats.accuracy()
        );
        let stats = Paragraph::new(stats_text)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Stats"));
        f.render_widget(stats, chunks[1]);

        if let Some(item) = dict.items.get(app.current_item_index) {
            let question = Paragraph::new(item.question.clone())
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true })
                .block(Block::default().borders(Borders::ALL).title("Question"));
            f.render_widget(question, chunks[2]);
        }

        if let Some((is_correct, msg)) = &app.feedback {
            let style = if *is_correct {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Red)
            };
            let feedback = Paragraph::new(msg.clone())
                .style(style)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Feedback"));
            f.render_widget(feedback, chunks[3]);
        } else {
            let input = Paragraph::new(app.input.clone())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Your Answer"));
            f.render_widget(input, chunks[3]);
        }

        let help = if app.feedback.is_some() {
            "Enter: Next Question | Esc: Back to Menu"
        } else {
            "Type your answer and press Enter | Esc: Back to Menu"
        };
        
        let help_widget = Paragraph::new(help)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(help_widget, chunks[4]);
    }
}

fn render_results(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title = Paragraph::new("Training Complete!")
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let results_text = format!(
        "Total Questions: {}\nCorrect: {}\nIncorrect: {}\nAccuracy: {:.1}%",
        app.stats.total,
        app.stats.correct,
        app.stats.incorrect,
        app.stats.accuracy()
    );
    
    let accuracy_color = if app.stats.accuracy() >= 80.0 {
        Color::Green
    } else if app.stats.accuracy() >= 60.0 {
        Color::Yellow
    } else {
        Color::Red
    };
    
    let results = Paragraph::new(results_text)
        .style(Style::default().fg(accuracy_color))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Results"));
    f.render_widget(results, chunks[1]);

    let help = Paragraph::new("Enter: Back to Menu | q: Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);
}
