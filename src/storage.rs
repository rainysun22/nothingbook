//! 数据存储模块
//!
//! 负责笔记数据的持久化存储和加载。
//! 使用 JSON 格式将笔记保存到用户主目录下的 .notes/ 文件夹中。

use crate::note::Note;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// 存储管理器
///
/// 处理所有与文件系统相关的操作，包括：
/// - 创建数据目录
/// - 保存笔记到 JSON 文件
/// - 从 JSON 文件加载笔记
/// - 删除笔记文件
#[derive(Clone)]
pub struct Storage {
    /// 数据目录的路径
    /// 通常是 ~/.notes/ (Linux/macOS) 或 C:\Users\用户名\.notes\ (Windows)
    data_dir: PathBuf,
}

impl Storage {
    /// 创建一个新的存储管理器实例
    ///
    /// # 返回值
    /// * `Ok(Storage)` - 成功创建的存储管理器
    /// * `Err` - 如果无法获取配置目录或创建数据目录失败
    ///
    /// # 示例
    /// ```
    /// let storage = Storage::new()?;
    /// ```
    pub fn new() -> Result<Self> {
        // 获取用户的配置目录
        // Linux: ~/.config
        // macOS: ~/Library/Application Support
        // Windows: C:\Users\用户名\AppData\Roaming
        let config_dir = dirs::config_dir().context("无法获取配置目录")?;

        // 在我们的配置目录下创建 notes 子目录
        let data_dir = config_dir.join("notes-app");

        // 确保数据目录存在，如果不存在则创建
        fs::create_dir_all(&data_dir).context("无法创建数据目录")?;

        Ok(Self { data_dir })
    }

    /// 保存笔记到文件
    ///
    /// 将笔记序列化为 JSON 并保存到数据目录。
    /// 每个笔记保存为单独的文件，文件名格式为: {note_id}.json
    ///
    /// # 参数
    /// * `note` - 要保存的笔记
    ///
    /// # 返回值
    /// * `Ok(())` - 保存成功
    /// * `Err` - 如果序列化或写入文件失败
    pub fn save_note(&self, note: &Note) -> Result<()> {
        // 构建文件路径: {data_dir}/{note_id}.json
        let file_path = self.data_dir.join(format!("{}.json", note.id));

        // 将笔记结构体序列化为格式化的 JSON 字符串
        let json = serde_json::to_string_pretty(note).context("序列化笔记失败")?;

        // 将 JSON 字符串写入文件
        fs::write(&file_path, json).context("写入笔记文件失败")?;

        Ok(())
    }

    /// 从文件加载所有笔记
    ///
    /// 扫描数据目录中的所有 .json 文件，将它们反序列化为 Note 结构体。
    ///
    /// # 返回值
    /// * `Ok(Vec<Note>)` - 成功加载的笔记列表（按更新时间倒序）
    /// * `Err` - 如果读取或解析文件失败
    pub fn load_all_notes(&self) -> Result<Vec<Note>> {
        let mut notes = Vec::new();

        // 读取数据目录中的所有条目
        let entries = fs::read_dir(&self.data_dir).context("无法读取数据目录")?;

        // 遍历目录中的每个文件
        for entry in entries {
            let entry = entry.context("读取目录条目失败")?;
            let path = entry.path();

            // 检查是否为 .json 文件
            if path.extension().map_or(false, |ext| ext == "json") {
                // 读取文件内容
                let content =
                    fs::read_to_string(&path).context(format!("读取文件失败: {:?}", path))?;

                // 将 JSON 字符串反序列化为 Note 结构体
                match serde_json::from_str::<Note>(&content) {
                    Ok(note) => notes.push(note),
                    Err(e) => {
                        eprintln!("解析笔记文件失败 {:?}: {}", path, e);
                        // 继续处理其他文件，不要因为一个文件损坏就停止
                    }
                }
            }
        }

        // 按更新时间倒序排序（最新的笔记在前）
        notes.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        Ok(notes)
    }

    /// 删除笔记文件
    ///
    /// 根据笔记 ID 删除对应的 JSON 文件。
    ///
    /// # 参数
    /// * `note_id` - 要删除的笔记的 ID
    ///
    /// # 返回值
    /// * `Ok(())` - 删除成功（即使文件不存在也返回成功）
    /// * `Err` - 如果删除操作失败
    pub fn delete_note(&self, note_id: &str) -> Result<()> {
        let file_path = self.data_dir.join(format!("{}.json", note_id));

        // 如果文件存在，则删除
        if file_path.exists() {
            fs::remove_file(&file_path).context("删除笔记文件失败")?;
        }

        Ok(())
    }

    /// 获取数据目录的路径
    ///
    /// # 返回值
    /// 数据目录的 PathBuf
    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir
    }
}
