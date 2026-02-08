use crate::note::Note;
use gpui::*;
use gpui_component::v_flex;

pub struct EditorView {
    current_note: Option<Note>,
}

impl EditorView {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { current_note: None }
    }

    pub fn load_note(&mut self, note: Note, cx: &mut Context<Self>) {
        self.current_note = Some(note.clone());
        cx.notify();
    }

    pub fn clear(&mut self) {
        self.current_note = None;
    }

    pub fn current_note_id(&self) -> Option<String> {
        self.current_note.as_ref().map(|n| n.id.clone())
    }
}

impl Render for EditorView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.current_note.is_none() {
            return div()
                .h_full()
                .w_full()
                .bg(gpui::rgb(0xffffff))
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_color(gpui::rgb(0x6b7280))
                        .child("选择或创建一个笔记开始编辑"),
                )
                .into_any_element();
        }

        let note = self.current_note.as_ref().unwrap();

        v_flex()
            .h_full()
            .flex_1()
            .bg(gpui::rgb(0xffffff))
            .child(
                v_flex()
                    .px_6()
                    .py_4()
                    .border_b_1()
                    .border_color(gpui::rgb(0xe5e7eb))
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .child(note.title.clone()),
                    )
                    .child(
                        div()
                            .mt_2()
                            .text_xs()
                            .text_color(gpui::rgb(0x6b7280))
                            .child(format!("创建于 {}", note.formatted_time())),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .p_6()
                    .child(div().text_base().child(note.content.clone())),
            )
            .child(
                div()
                    .px_6()
                    .py_2()
                    .border_t_1()
                    .border_color(gpui::rgb(0xe5e7eb))
                    .child(
                        div()
                            .text_xs()
                            .text_color(gpui::rgb(0x6b7280))
                            .child("提示：使用 Markdown 语法格式化文本"),
                    ),
            )
            .into_any_element()
    }
}
