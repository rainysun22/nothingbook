use crate::{
    note::Note,
    storage::Storage,
    views::{
        editor::EditorView,
        sidebar::{
            SidebarDeleteNote, SidebarEvent, SidebarNewNote, SidebarRenameNote, SidebarView,
        },
    },
};
use gpui::*;
use gpui_component::h_flex;
use std::sync::Arc;

pub struct AppView {
    sidebar: Entity<SidebarView>,
    editor: Entity<EditorView>,
    notes: Arc<Vec<Note>>,
    selected_note_id: Option<String>,
    storage: Storage,
}

impl AppView {
    pub fn new(cx: &mut Context<Self>) -> anyhow::Result<Self> {
        let storage = Storage::new()?;
        let notes = storage.load_all_notes()?;
        let notes_arc = Arc::new(notes);

        let sidebar = cx.new(|_cx| {
            let notes_clone = notes_arc.to_vec();
            SidebarView::new(notes_clone)
        });

        let editor = cx.new(|cx| EditorView::new(cx));

        let selected_note_id = notes_arc.first().map(|n| n.id.clone());

        let app = Self {
            sidebar,
            editor,
            notes: notes_arc,
            selected_note_id,
            storage,
        };

        if let Some(ref note_id) = app.selected_note_id {
            if let Some(note) = app.notes.iter().find(|n| &n.id == note_id) {
                app.editor.update(cx, |editor, cx| {
                    editor.load_note(note.clone(), cx);
                });
                app.sidebar.update(cx, |sidebar, _cx| {
                    sidebar.set_selected(Some(note_id.clone()));
                });
            }
        }

        cx.subscribe(&app.sidebar, |this, _, event: &SidebarEvent, cx| {
            this.select_note(event.0.clone(), cx);
        })
        .detach();

        cx.subscribe(&app.sidebar, |this, _, _: &SidebarNewNote, cx| {
            this.create_new_note(cx);
        })
        .detach();

        cx.subscribe(&app.sidebar, |this, _, event: &SidebarDeleteNote, cx| {
            this.delete_note(event.0.clone(), cx);
        })
        .detach();

        cx.subscribe(&app.sidebar, |this, _, event: &SidebarRenameNote, cx| {
            this.rename_note(event.0.clone(), event.1.clone(), cx);
        })
        .detach();

        Ok(app)
    }

    fn create_new_note(&mut self, cx: &mut Context<Self>) {
        let new_note = Note::new();
        let note_id = new_note.id.clone();

        if let Err(e) = self.storage.save_note(&new_note) {
            eprintln!("保存新笔记失败: {}", e);
            return;
        }

        let mut notes = (*self.notes).clone();
        notes.insert(0, new_note.clone());
        self.notes = Arc::new(notes);

        self.sidebar.update(cx, |sidebar, _cx| {
            sidebar.update_notes((*self.notes).clone());
            sidebar.set_selected(Some(note_id.clone()));
        });

        self.editor.update(cx, |editor, cx| {
            editor.load_note(new_note, cx);
        });

        self.selected_note_id = Some(note_id);
        cx.notify();
    }

    fn delete_note(&mut self, note_id: String, cx: &mut Context<Self>) {
        if let Err(e) = self.storage.delete_note(&note_id) {
            eprintln!("删除笔记失败: {}", e);
            return;
        }

        let mut notes = (*self.notes).clone();
        notes.retain(|n| n.id != note_id);
        self.notes = Arc::new(notes);

        self.sidebar.update(cx, |sidebar, _cx| {
            sidebar.update_notes((*self.notes).clone());
            sidebar.set_editing(None);
            sidebar.set_selected(None);
        });

        self.editor.update(cx, |editor, _cx| {
            editor.clear();
        });

        self.selected_note_id = None;
        cx.notify();
    }

    fn rename_note(&mut self, note_id: String, new_title: String, cx: &mut Context<Self>) {
        let mut notes = (*self.notes).clone();
        if let Some(note) = notes.iter_mut().find(|n| n.id == note_id) {
            note.title = new_title.clone();
            note.updated_at = chrono::Local::now();

            if let Err(e) = self.storage.save_note(note) {
                eprintln!("重命名笔记失败: {}", e);
                return;
            }

            self.notes = Arc::new(notes);

            self.sidebar.update(cx, |sidebar, _cx| {
                sidebar.update_notes((*self.notes).clone());
                sidebar.set_editing(None);
            });

            if Some(&note_id) == self.selected_note_id.as_ref() {
                self.editor.update(cx, |editor, cx| {
                    if let Some(note) = self.notes.iter().find(|n| n.id == note_id) {
                        editor.load_note(note.clone(), cx);
                    }
                });
            }

            cx.notify();
        }
    }

    fn select_note(&mut self, note_id: String, cx: &mut Context<Self>) {
        if let Some(note) = self.notes.iter().find(|n| n.id == note_id) {
            self.editor.update(cx, |editor, cx| {
                editor.load_note(note.clone(), cx);
            });
            self.sidebar.update(cx, |sidebar, _cx| {
                sidebar.set_selected(Some(note_id.clone()));
            });
            self.selected_note_id = Some(note_id);
            cx.notify();
        }
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .bg(gpui::rgb(0xffffff))
            .child(div().w(px(280.0)).h_full().child(self.sidebar.clone()))
            .child(div().flex_1().h_full().child(self.editor.clone()))
    }
}
