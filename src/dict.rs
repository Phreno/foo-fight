use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dictionary {
    pub name: String,
    #[serde(default)]
    pub version: i32,
    #[serde(default = "default_language")]
    pub language: String,
    pub items: Vec<DictItem>,
}

fn default_language() -> String {
    "en".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictItem {
    pub id: String,
    pub prompt: String,
    pub answer: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub difficulty: i32,
}

pub struct DictMeta {
    pub path: PathBuf,
    pub name: String,
}

impl Dictionary {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read dictionary file: {:?}", path.as_ref()))?;
        
        let dict: Dictionary = toml::from_str(&content)
            .with_context(|| format!("Failed to parse dictionary file: {:?}", path.as_ref()))?;
        
        if dict.items.is_empty() {
            anyhow::bail!("Dictionary has no items");
        }
        
        Ok(dict)
    }

    pub fn validate_answer(&self, item_index: usize, user_input: &str) -> bool {
        if item_index >= self.items.len() {
            return false;
        }

        let item = &self.items[item_index];
        let trimmed_input = user_input.trim();
        let trimmed_answer = item.answer.trim();

        if trimmed_input == trimmed_answer {
            return true;
        }

        for alias in &item.aliases {
            if trimmed_input == alias.trim() {
                return true;
            }
        }

        false
    }
}

pub fn list_dictionaries<P: AsRef<Path>>(dictionaries_path: P) -> Result<Vec<DictMeta>> {
    let path = dictionaries_path.as_ref();
    
    if !path.exists() {
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create dictionaries directory: {:?}", path))?;
        return Ok(Vec::new());
    }

    let mut dictionaries = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let name = match Dictionary::from_file(&file_path) {
                Ok(dict) => dict.name,
                Err(_) => file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown")
                    .to_string(),
            };

            dictionaries.push(DictMeta {
                path: file_path,
                name,
            });
        }
    }

    Ok(dictionaries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_answer_exact() {
        let dict = Dictionary {
            name: "test".to_string(),
            version: 1,
            language: "en".to_string(),
            items: vec![DictItem {
                id: "test1".to_string(),
                prompt: "Test prompt".to_string(),
                answer: "git status".to_string(),
                aliases: vec![],
                tags: vec![],
                difficulty: 1,
            }],
        };

        assert!(dict.validate_answer(0, "git status"));
        assert!(dict.validate_answer(0, "  git status  "));
        assert!(!dict.validate_answer(0, "git"));
    }

    #[test]
    fn test_validate_answer_with_aliases() {
        let dict = Dictionary {
            name: "test".to_string(),
            version: 1,
            language: "en".to_string(),
            items: vec![DictItem {
                id: "test1".to_string(),
                prompt: "Test prompt".to_string(),
                answer: "git status".to_string(),
                aliases: vec!["git st".to_string()],
                tags: vec![],
                difficulty: 1,
            }],
        };

        assert!(dict.validate_answer(0, "git status"));
        assert!(dict.validate_answer(0, "git st"));
        assert!(!dict.validate_answer(0, "git"));
    }
}
