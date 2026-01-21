use crate::dict::Dictionary;

pub struct TrainingSession {
    pub current_index: usize,
    pub correct_count: usize,
    pub incorrect_count: usize,
    pub streak: usize,
    pub items_order: Vec<usize>,
}

impl TrainingSession {
    pub fn new(dictionary: &Dictionary, shuffle: bool) -> Self {
        let mut items_order: Vec<usize> = (0..dictionary.items.len()).collect();

        if shuffle {
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            items_order.shuffle(&mut rng);
        }

        TrainingSession {
            current_index: 0,
            correct_count: 0,
            incorrect_count: 0,
            streak: 0,
            items_order,
        }
    }

    pub fn total_items(&self) -> usize {
        self.items_order.len()
    }

    /// Returns the current item index in the dictionary.
    /// Returns 0 as a safe fallback if current_index is out of bounds,
    /// though this should never happen in normal operation due to is_complete() checks.
    pub fn current_item_index(&self) -> usize {
        debug_assert!(
            self.current_index < self.items_order.len(),
            "current_index out of bounds: {} >= {}",
            self.current_index,
            self.items_order.len()
        );
        if self.current_index < self.items_order.len() {
            self.items_order[self.current_index]
        } else {
            0
        }
    }

    pub fn mark_correct(&mut self) {
        self.correct_count += 1;
        self.streak += 1;
    }

    pub fn mark_incorrect(&mut self) {
        self.incorrect_count += 1;
        self.streak = 0;
    }

    pub fn next_item(&mut self) {
        if self.current_index < self.items_order.len() {
            self.current_index += 1;
        }
    }

    pub fn is_complete(&self) -> bool {
        self.current_index >= self.items_order.len()
    }

    pub fn success_rate(&self) -> f32 {
        let total = self.correct_count + self.incorrect_count;
        if total == 0 {
            0.0
        } else {
            (self.correct_count as f32 / total as f32) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dict::{DictItem, Dictionary};

    fn create_test_dictionary() -> Dictionary {
        Dictionary {
            name: "test".to_string(),
            version: 1,
            language: "en".to_string(),
            items: vec![
                DictItem {
                    id: "1".to_string(),
                    prompt: "Prompt 1".to_string(),
                    answer: "answer 1".to_string(),
                    aliases: vec![],
                    tags: vec![],
                    difficulty: 1,
                },
                DictItem {
                    id: "2".to_string(),
                    prompt: "Prompt 2".to_string(),
                    answer: "answer 2".to_string(),
                    aliases: vec![],
                    tags: vec![],
                    difficulty: 1,
                },
            ],
        }
    }

    #[test]
    fn test_session_creation() {
        let dict = create_test_dictionary();
        let session = TrainingSession::new(&dict, false);

        assert_eq!(session.total_items(), 2);
        assert_eq!(session.current_index, 0);
        assert_eq!(session.correct_count, 0);
    }

    #[test]
    fn test_mark_correct() {
        let dict = create_test_dictionary();
        let mut session = TrainingSession::new(&dict, false);

        session.mark_correct();
        assert_eq!(session.correct_count, 1);
        assert_eq!(session.streak, 1);
    }

    #[test]
    fn test_success_rate() {
        let dict = create_test_dictionary();
        let mut session = TrainingSession::new(&dict, false);

        session.mark_correct();
        session.mark_incorrect();
        assert_eq!(session.success_rate(), 50.0);
    }
}
