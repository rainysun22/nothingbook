//! 笔记数据模型模块
//!
//! 这个模块定义了笔记应用的核心数据结构。
//! Note 结构体代表单个笔记，包含标题、内容、创建时间等信息。

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 笔记结构体
///
/// 这是应用的核心数据模型，每个实例代表一条笔记。
/// 使用 serde 进行 JSON 序列化/反序列化。
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Note {
    /// 笔记的唯一标识符
    /// 使用 UUID v4 生成，确保全局唯一性
    pub id: String,

    /// 笔记标题
    /// 用户可编辑，默认为"新建笔记"
    pub title: String,

    /// 笔记内容
    /// 支持 Markdown 格式的纯文本
    pub content: String,

    /// 创建时间
    /// 记录笔记首次创建的本地时间
    pub created_at: DateTime<Local>,

    /// 最后更新时间
    /// 每次编辑后自动更新
    pub updated_at: DateTime<Local>,
}

impl Note {
    /// 创建一个新的笔记实例
    ///
    /// # 返回值
    /// 返回一个带有默认标题"新建笔记"的新 Note
    ///
    /// # 示例
    /// ```
    /// let note = Note::new();
    /// assert_eq!(note.title, "新建笔记");
    /// ```
    pub fn new() -> Self {
        let now = Local::now();
        Self {
            // 生成一个新的 UUID 作为 ID
            id: Uuid::new_v4().to_string(),
            // 默认标题
            title: "新建笔记".to_string(),
            // 空内容
            content: String::new(),
            // 当前时间作为创建时间
            created_at: now,
            // 当前时间作为更新时间
            updated_at: now,
        }
    }

    /// 从现有数据创建笔记（用于从文件加载）
    ///
    /// # 参数
    /// * `id` - 笔记的 UUID
    /// * `title` - 笔记标题
    /// * `content` - 笔记内容
    /// * `created_at` - 创建时间
    /// * `updated_at` - 更新时间
    pub fn from_data(
        id: String,
        title: String,
        content: String,
        created_at: DateTime<Local>,
        updated_at: DateTime<Local>,
    ) -> Self {
        Self {
            id,
            title,
            content,
            created_at,
            updated_at,
        }
    }

    /// 更新笔记内容
    ///
    /// 更新内容时会自动更新 updated_at 时间戳
    ///
    /// # 参数
    /// * `new_content` - 新的笔记内容
    pub fn update_content(&mut self, new_content: String) {
        self.content = new_content;
        self.updated_at = Local::now();
    }

    /// 更新笔记标题
    ///
    /// 更新标题时会自动更新 updated_at 时间戳
    ///
    /// # 参数
    /// * `new_title` - 新的笔记标题
    pub fn update_title(&mut self, new_title: String) {
        self.title = new_title;
        self.updated_at = Local::now();
    }

    /// 获取笔记的简短预览（用于侧边栏显示）
    ///
    /// # 返回值
    /// 返回内容的前50个字符，如果内容为空则返回"无内容"
    pub fn preview(&self) -> String {
        if self.content.is_empty() {
            "无内容".to_string()
        } else {
            // 提取前50个字符
            let preview: String = self.content.chars().take(50).collect();
            // 如果内容被截断，添加省略号
            if self.content.len() > 50 {
                format!("{}...", preview)
            } else {
                preview
            }
        }
    }

    /// 获取格式化的更新时间字符串
    ///
    /// # 返回值
    /// 返回格式如 "2025-02-08 14:30" 的时间字符串
    pub fn formatted_time(&self) -> String {
        self.updated_at.format("%Y-%m-%d %H:%M").to_string()
    }
}

impl Default for Note {
    /// 实现 Default trait，使 Note 可以使用 default() 方法
    fn default() -> Self {
        Self::new()
    }
}
