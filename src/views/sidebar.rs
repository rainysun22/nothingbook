use crate::note::Note;
use gpui::*;
use gpui_component::{button::Button, input::Input, input::InputEvent, input::InputState, v_flex};
use std::collections::HashMap;

#[derive(Clone)]
pub struct SidebarEvent(pub String);

#[derive(Clone)]
pub struct SidebarNewNote;

#[derive(Clone)]
pub struct SidebarDeleteNote(pub String);

#[derive(Clone)]
pub struct SidebarRenameNote(pub String, pub String);

pub struct SidebarView {
    notes: HashMap<String, Note>,
    selected_note_id: Option<String>,
    editing_note_id: Option<String>,
    rename_title: String,
    input_state: Option<Entity<InputState>>,
}

impl SidebarView {
    pub fn new(notes: &HashMap<String, Note>) -> Self {
        Self {
            notes: notes.clone(),
            selected_note_id: None,
            editing_note_id: None,
            rename_title: String::new(),
            input_state: None,
        }
    }

    pub fn update_notes(&mut self, notes: HashMap<String, Note>) {
        self.notes = notes;
    }

    pub fn set_selected(&mut self, note_id: Option<String>) {
        self.selected_note_id = note_id;
    }

    pub fn set_editing(&mut self, note_id: Option<String>) {
        self.editing_note_id = note_id;
    }

    pub fn start_editing(&mut self, note_id: String, window: &mut Window, cx: &mut Context<Self>) {
        self.editing_note_id = Some(note_id.clone());
        if let Some(note) = self.notes.get(&note_id) {
            self.rename_title = note.title.clone();
            let input_state = cx.new(|cx| InputState::new(window, cx));
            input_state.update(cx, |state, cx| {
                state.set_value(note.title.clone(), window, cx);
                state.focus(window, cx);
            });
            self.input_state = Some(input_state.clone());

            cx.subscribe(
                &input_state,
                move |this, _, event: &InputEvent, cx| match event {
                    InputEvent::Change => {
                        if let Some(ref state) = this.input_state {
                            let value = state.read(cx).value().to_string();
                            this.rename_title = value;
                            cx.notify();
                        }
                    }
                    InputEvent::PressEnter { .. } => {
                        if let Some((id, title)) = this.confirm_rename() {
                            cx.emit(SidebarRenameNote(id, title));
                        }
                    }
                    InputEvent::Focus => {}
                    InputEvent::Blur => {}
                },
            )
            .detach();
        }
    }

    pub fn is_editing(&self, note_id: &str) -> bool {
        self.editing_note_id.as_ref() == Some(&note_id.to_string())
    }

    pub fn cancel_rename(&mut self) {
        self.editing_note_id = None;
        self.rename_title.clear();
        self.input_state = None;
    }

    pub fn confirm_rename(&mut self) -> Option<(String, String)> {
        if let Some(note_id) = self.editing_note_id.take() {
            if !self.rename_title.is_empty() {
                let title = self.rename_title.clone();
                self.rename_title.clear();
                self.input_state = None;
                return Some((note_id, title));
            }
            self.rename_title.clear();
            self.input_state = None;
        }
        None
    }
}

impl EventEmitter<SidebarEvent> for SidebarView {}
impl EventEmitter<SidebarNewNote> for SidebarView {}
impl EventEmitter<SidebarDeleteNote> for SidebarView {}
impl EventEmitter<SidebarRenameNote> for SidebarView {}

impl Render for SidebarView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .h_full()
            .w(px(280.0))
            .bg(gpui::rgb(0xf9fafb))
            .border_r_1()
            .border_color(gpui::rgb(0xe5e7eb))
            .child(
                v_flex()
                    .px_4()
                    .py_3()
                    .border_b_1()
                    .border_color(gpui::rgb(0xe5e7eb))
                    .child(
                        div()
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
                                    cx.emit(SidebarNewNote);
                                },
                            ))),
                    )
                    .child(
                        div()
                            .mt_1()
                            .text_sm()
                            .text_color(gpui::rgb(0x6b7280))
                            .child(format!("{} 条笔记", self.notes.len())),
                    ),
            )
            .child(div().flex_1().children(self.notes.iter().map(|note| {
                let is_selected = self
                    .selected_note_id
                    .as_ref()
                    .map_or(false, |id| id == note.0);
                let is_editing = self.is_editing(&note.0);

                let note_id = note.0.clone();
                let note_title = &note.1.title;
                let note_id_for_rename = note.0;
                let note_id_for_delete = note.0;

                if is_editing {
                    div()
                        .id(SharedString::from(note_id))
                        .px_4()
                        .py_3()
                        .border_b_1()
                        .border_color(gpui::rgb(0xe5e7eb))
                        .bg(gpui::rgb(0xffffff))
                        .child(
                            div()
                                .mb_2()
                                .text_sm()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(gpui::rgb(0x374151))
                                .child("重命名笔记"),
                        )
                        .child(
                            Input::new(self.input_state.as_ref().unwrap())
                                .bordered(false)
                                .w_full(),
                        )
                        .child(
                            v_flex()
                                .mt_2()
                                .gap_2()
                                .flex_row()
                                .child(Button::new("confirm-rename").label("确认").on_click(
                                    cx.listener({
                                        move |this, _, _window, cx| {
                                            if let Some((id, title)) = this.confirm_rename() {
                                                cx.emit(SidebarRenameNote(id, title));
                                            }
                                        }
                                    }),
                                ))
                                .child(Button::new("cancel-rename").label("取消").on_click(
                                    cx.listener(move |this, _, _window, _cx| {
                                        this.cancel_rename();
                                    }),
                                )),
                        )
                } else {
                    div()
                        .id(SharedString::from(&note_id))
                        .px_4()
                        .py_3()
                        .border_b_1()
                        .border_color(gpui::rgb(0xe5e7eb))
                        .cursor_pointer()
                        .bg(if is_selected {
                            gpui::rgb(0x3b82f6)
                        } else {
                            gpui::rgb(0xf9fafb)
                        })
                        .on_click(cx.listener({
                            move |this, _, _window, cx| {
                                this.selected_note_id = Some(note_id.clone());
                                cx.emit(SidebarEvent(note_id.clone()));
                            }
                        }))
                        .child(
                            v_flex()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(if is_selected {
                                            gpui::rgb(0xffffff)
                                        } else {
                                            gpui::rgb(0x111827)
                                        })
                                        .truncate()
                                        .child(note_title.clone()),
                                )
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child(Button::new("rename-btn").label("重命名").on_click(
                                            cx.listener({
                                                let note_id = note_id_for_rename.clone();
                                                move |this, _, window, cx| {
                                                    this.start_editing(note_id.clone(), window, cx);
                                                }
                                            }),
                                        ))
                                        .child(Button::new("delete-btn").label("删除").on_click(
                                            cx.listener({
                                                let note_id = note_id_for_delete.clone();
                                                move |_, _, _window, cx| {
                                                    cx.emit(SidebarDeleteNote(note_id.clone()));
                                                }
                                            }),
                                        )),
                                ),
                        )
                        .child(
                            div()
                                .mt_1()
                                .text_xs()
                                .text_color(if is_selected {
                                    gpui::rgb(0xbfdbfe)
                                } else {
                                    gpui::rgb(0x6b7280)
                                })
                                .truncate()
                                .child(note.1.preview()),
                        )
                        .child(
                            div()
                                .mt_1()
                                .text_xs()
                                .text_color(if is_selected {
                                    gpui::rgb(0x93c5fd)
                                } else {
                                    gpui::rgb(0x9ca3af)
                                })
                                .child(note.1.formatted_time()),
                        )
                }
            })))
    }
}
