mod dictionary;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use std::path::PathBuf;

use dictionary::load_dictionaries;
use ui::{App, AppState};

const DICT_PATH: &str = "dictionaries";
const NO_DICT_WARNING: &str = "Warning: No dictionaries found in dictionaries/ folder";
const NO_DICT_HELP: &str = "Please add some .toml dictionary files to get started.";

fn main() -> Result<()> {
    // Load dictionaries
    let dict_path = PathBuf::from(DICT_PATH);
    let dictionaries = load_dictionaries(&dict_path)?;

    if dictionaries.is_empty() {
        eprintln!("{}", NO_DICT_WARNING);
        eprintln!("{}", NO_DICT_HELP);
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let app = App::new(dictionaries);
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        if let Event::Key(key) = event::read()? {
            // Only process key press events, not release
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match app.state {
                AppState::SelectDictionary => {
                    match key.code {
                        KeyCode::Char('q') => {
                            app.should_quit = true;
                        }
                        KeyCode::Down => {
                            app.next_dict();
                        }
                        KeyCode::Up => {
                            app.prev_dict();
                        }
                        KeyCode::Enter => {
                            if !app.dictionaries.is_empty() {
                                app.select_dictionary();
                            }
                        }
                        _ => {}
                    }
                }
                AppState::Training => {
                    if app.feedback.is_some() {
                        match key.code {
                            KeyCode::Enter => {
                                app.next_question();
                            }
                            KeyCode::Esc => {
                                app.back_to_menu();
                            }
                            _ => {}
                        }
                    } else {
                        match key.code {
                            KeyCode::Char(c) => {
                                app.input.push(c);
                            }
                            KeyCode::Backspace => {
                                app.input.pop();
                            }
                            KeyCode::Enter => {
                                if !app.input.is_empty() {
                                    app.submit_answer();
                                }
                            }
                            KeyCode::Esc => {
                                app.back_to_menu();
                            }
                            _ => {}
                        }
                    }
                }
                AppState::Results => {
                    match key.code {
                        KeyCode::Char('q') => {
                            app.should_quit = true;
                        }
                        KeyCode::Enter => {
                            app.back_to_menu();
                        }
                        _ => {}
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
