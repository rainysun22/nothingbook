use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Note {
    pub fn new() -> Self {
        let now = Local::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title: "新建笔记".to_string(),
            content: String::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_title(&mut self, new_title: String) {
        self.title = new_title;
        self.updated_at = Local::now();
    }

    pub fn preview(&self) -> String {
        if self.content.is_empty() {
            "无内容".to_string()
        } else {
            let preview: String = self.content.chars().take(50).collect();
            if self.content.len() > 50 {
                format!("{}...", preview)
            } else {
                preview
            }
        }
    }

    pub fn formatted_time(&self) -> String {
        self.updated_at.format("%Y-%m-%d %H:%M").to_string()
    }
}

impl Default for Note {
    fn default() -> Self {
        Self::new()
    }
}
