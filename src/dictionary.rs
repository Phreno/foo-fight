use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

#[derive(Debug, Deserialize, Clone)]
pub struct Dictionary {
    pub name: String,
    pub description: String,
    pub items: Vec<DictionaryItem>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DictionaryItem {
    pub question: String,
    pub command: String,
    pub aliases: Vec<String>,
}

impl DictionaryItem {
    pub fn is_correct(&self, answer: &str) -> bool {
        let answer = answer.trim();
        if answer == self.command {
            return true;
        }
        self.aliases.iter().any(|alias| alias == answer)
    }
}

pub fn load_dictionaries(dict_path: &Path) -> Result<Vec<(PathBuf, Dictionary)>> {
    let mut dictionaries = Vec::new();
    
    if !dict_path.exists() {
        return Ok(dictionaries);
    }

    for entry in fs::read_dir(dict_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let content = fs::read_to_string(&path)
                .with_context(|| format!("Failed to read {}", path.display()))?;
            let dict: Dictionary = toml::from_str(&content)
                .with_context(|| format!("Failed to parse {}", path.display()))?;
            dictionaries.push((path, dict));
        }
    }
    
    Ok(dictionaries)
}
