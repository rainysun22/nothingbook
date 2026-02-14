use crate::note_list::NoteList;
use gpui::*;
use gpui_component::{button::Button, v_flex};

pub enum SidebarEvent {
    CreateNote,
    SelectNote(u128),
    DeleteNote(u128),
}

pub struct SidebarView {
    notes: Entity<NoteList>,
    selected_note_id: Option<u128>,
}

impl SidebarView {
    pub fn new(notes: Entity<NoteList>) -> Self {
        Self {
            notes,
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
        let notes = self.notes.read(cx);
        let note_list = notes.get_all();

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
                    .child(Button::new("new-note").label("新建").on_click(cx.listener(
                        |_, _, _window, cx| {
                            cx.emit(SidebarEvent::CreateNote);
                        },
                    ))),
            )
            .child(
                v_flex()
                    .flex_1()
                    .overflow_hidden()
                    .children(note_list.iter().map(|note| {
                        let note_id = note.id;
                        let is_selected = self.selected_note_id == Some(note_id);
                        div()
                            .p_3()
                            .border_b_1()
                            .border_color(gpui::rgb(0xe5e7eb))
                            .cursor_pointer()
                            .bg(if is_selected {
                                gpui::rgb(0xe0e7ff)
                            } else {
                                gpui::rgb(0xf9fafb)
                            })
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |_, _, _window, cx| {
                                    cx.emit(SidebarEvent::SelectNote(note_id));
                                }),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .items_center()
                                    .child(
                                        div()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_base()
                                            .child(note.title.clone()),
                                    )
                                    .child(
                                        Button::new("delete-note")
                                            .label("删除")
                                            .compact()
                                            .on_click(cx.listener(move |_, _, _window, cx| {
                                                cx.emit(SidebarEvent::DeleteNote(note_id));
                                            })),
                                    ),
                            )
                            .child(div().mt_1().text_sm().child(note.preview()))
                            .child(
                                div()
                                    .mt_1()
                                    .text_xs()
                                    .text_color(gpui::rgb(0x9ca3af))
                                    .child(note.formatted_time()),
                            )
                    })),
            )
            .into_any_element()
    }
}
