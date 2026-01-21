use anyhow::Result;
use std::path::PathBuf;

use crate::dict::{list_dictionaries, DictMeta, Dictionary};
use crate::engine::TrainingSession;

#[derive(Debug, PartialEq)]
pub enum AppState {
    SelectDictionary,
    Training,
    #[allow(dead_code)]
    Quit,
}

pub struct App {
    pub state: AppState,
    pub dictionaries: Vec<DictMeta>,
    pub selected_dict_index: usize,
    pub current_dictionary: Option<Dictionary>,
    pub session: Option<TrainingSession>,
    pub input: String,
    pub show_feedback: bool,
    pub is_correct: bool,
    pub feedback_message: String,
}

impl App {
    pub fn new() -> Result<Self> {
        let dictionaries_path = PathBuf::from("dictionaries");
        let dictionaries = list_dictionaries(&dictionaries_path)?;

        Ok(App {
            state: AppState::SelectDictionary,
            dictionaries,
            selected_dict_index: 0,
            current_dictionary: None,
            session: None,
            input: String::new(),
            show_feedback: false,
            is_correct: false,
            feedback_message: String::new(),
        })
    }

    pub fn previous_dictionary(&mut self) {
        if !self.dictionaries.is_empty() && self.selected_dict_index > 0 {
            self.selected_dict_index -= 1;
        }
    }

    pub fn next_dictionary(&mut self) {
        if !self.dictionaries.is_empty() && self.selected_dict_index < self.dictionaries.len() - 1 {
            self.selected_dict_index += 1;
        }
    }

    pub fn select_dictionary(&mut self) -> Result<()> {
        if self.dictionaries.is_empty() {
            return Ok(());
        }

        let dict_meta = &self.dictionaries[self.selected_dict_index];
        let dictionary = Dictionary::from_file(&dict_meta.path)?;

        let session = TrainingSession::new(&dictionary, true);

        self.current_dictionary = Some(dictionary);
        self.session = Some(session);
        self.state = AppState::Training;
        self.input.clear();
        self.show_feedback = false;

        Ok(())
    }

    pub fn back_to_selection(&mut self) {
        self.state = AppState::SelectDictionary;
        self.current_dictionary = None;
        self.session = None;
        self.input.clear();
        self.show_feedback = false;
    }

    pub fn add_char(&mut self, c: char) {
        if !self.show_feedback {
            self.input.push(c);
        }
    }

    pub fn delete_char(&mut self) {
        if !self.show_feedback {
            self.input.pop();
        }
    }

    pub fn validate_answer(&mut self) -> Result<()> {
        if self.show_feedback {
            // If showing feedback, Enter moves to next
            if self.is_correct {
                self.next_item();
            }
            return Ok(());
        }

        if let (Some(dict), Some(session)) = (&self.current_dictionary, &mut self.session) {
            let item_index = session.current_item_index();
            let is_correct = dict.validate_answer(item_index, &self.input);

            self.is_correct = is_correct;
            self.show_feedback = true;

            if is_correct {
                session.mark_correct();
                self.feedback_message = "✓ Correct!".to_string();
            } else {
                session.mark_incorrect();
                let correct_answer = &dict.items[item_index].answer;
                self.feedback_message = format!("✗ Incorrect. Expected: {}", correct_answer);
            }
        }

        Ok(())
    }

    pub fn retry(&mut self) {
        self.input.clear();
        self.show_feedback = false;
        self.feedback_message.clear();
    }

    pub fn skip(&mut self) {
        self.next_item();
    }

    fn next_item(&mut self) {
        if let Some(session) = &mut self.session {
            session.next_item();

            if session.is_complete() {
                self.back_to_selection();
            } else {
                self.input.clear();
                self.show_feedback = false;
                self.feedback_message.clear();
            }
        }
    }

    pub fn get_current_prompt(&self) -> Option<&str> {
        if let (Some(dict), Some(session)) = (&self.current_dictionary, &self.session) {
            let item_index = session.current_item_index();
            Some(&dict.items[item_index].prompt)
        } else {
            None
        }
    }
}
