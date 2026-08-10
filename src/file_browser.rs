/* FastNote Rust/GTK4 Edition — File browser */

use std::fs;
use std::path::Path;

pub struct FileBrowser {
    pub current_dir: String,
    pub entries: Vec<String>,
    pub selected: usize,
}

impl FileBrowser {
    pub fn new(start: &str) -> Self {
        Self {
            current_dir: start.to_string(),
            entries: Vec::new(),
            selected: 0,
        }
    }

    pub fn refresh(&mut self) {
        self.entries.clear();
        self.selected = 0;

        if self.current_dir.len() > 1 {
            self.entries.push("..".to_string());
        }

        if let Ok(entries) = fs::read_dir(&self.current_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if !name.starts_with('.') {
                        let full = format!("{}/{}", self.current_dir, name);
                        self.entries.push(full);
                    }
                }
            }
        }
    }

    pub fn selected_path(&self) -> Option<&str> {
        if self.selected < self.entries.len() {
            Some(&self.entries[self.selected])
        } else {
            None
        }
    }
}
