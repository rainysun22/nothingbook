use crate::note::Note;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
#[derive(Clone)]
pub struct Storage {
    data_dir: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self> {
        // 获取用户的配置目录
        // Linux: ~/.config
        // macOS: ~/Library/Application Support
        // Windows: C:\Users\用户名\AppData\Roaming
        let config_dir = dirs::config_dir().context("无法获取配置目录")?;
        let data_dir = config_dir.join("notes-app");
        fs::create_dir_all(&data_dir).context("无法创建数据目录")?;
        Ok(Self { data_dir })
    }

    pub fn save_note(&self, note: &Note) -> Result<()> {
        let file_path = self.data_dir.join(format!("{}.json", note.id));
        let json = serde_json::to_string_pretty(note).context("序列化笔记失败")?;
        fs::write(&file_path, json).context("写入笔记文件失败")?;
        Ok(())
    }

    pub fn load_all_notes(&self) -> Result<HashMap<String, Note>> {
        let mut notes = HashMap::new();
        let entries = fs::read_dir(&self.data_dir).context("无法读取数据目录")?;

        for entry in entries {
            let entry = entry.context("读取目录条目失败")?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "json") {
                let content =
                    fs::read_to_string(&path).context(format!("读取文件失败: {:?}", path))?;
                match serde_json::from_str::<Note>(&content) {
                    Ok(note) => {
                        notes.insert(note.id.clone(), note);
                    },
                    Err(e) => {
                        eprintln!("解析笔记文件失败 {:?}: {}", path, e);
                    }
                }
            }
        }
        Ok(notes)
    }

    pub fn delete_note(&self, note_id: &str) -> Result<()> {
        let file_path = self.data_dir.join(format!("{}.json", note_id));
        if file_path.exists() {
            fs::remove_file(&file_path).context("删除笔记文件失败")?;
        }
        Ok(())
    }
}
