use gpui::*;
use gpui_component::{button::Button, v_flex};

pub enum SidebarEvent {
    CreateNote,
    SelectNote(u128),
    DeleteNote(u128),
}

pub struct SidebarView {
    selected_note_id: Option<u128>,
}

impl SidebarView {
    pub fn new() -> Self {
        Self {
            selected_note_id: None,
        }
    }

    pub fn set_selected(&mut self, note_id: Option<u128>) {
        self.selected_note_id = note_id;
    }
}

impl EventEmitter<SidebarEvent> for SidebarView {}

impl Render for SidebarView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .h_full()
            .w(px(280.0))
            .bg(gpui::rgb(0xf9fafb))
            .border_r_1()
            .border_color(gpui::rgb(0xe5e7eb))
            .child(
                div()
                    .px_4()
                    .py_3()
                    .border_b_1()
                    .border_color(gpui::rgb(0xe5e7eb))
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("我的笔记"),
                    )
                    .child(
                        Button::new("new-note")
                            .label("新建")
                            .on_click(cx.listener(|_, _, _window, cx| {
                                cx.emit(SidebarEvent::CreateNote);
                            })),
                    ),
            )
    }
}
