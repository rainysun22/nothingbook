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
use std::collections::HashMap;

pub struct AppView {
    sidebar: Entity<SidebarView>,
    editor: Entity<EditorView>,
    notes: HashMap<String, Note>,
    storage: Storage,
}

impl AppView {
    pub fn new(cx: &mut Context<Self>) -> anyhow::Result<Self> {
        let storage = Storage::new()?;
        let notes = storage.load_all_notes()?;

        let sidebar = cx.new(|_cx| {
            SidebarView::new(&notes)
        });

        let editor = cx.new(|cx| EditorView::new(cx));

        let selected_note_id = notes.keys().next().map(|k| k.clone());

        let app = Self {
            sidebar,
            editor,
            notes: notes,
            storage,
        };

        if let Some(note_id) = &selected_note_id {
            if let Some(note) = app.notes.get(note_id) {
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
        let note_id = &new_note.id;

        if let Err(e) = self.storage.save_note(&new_note) {
            eprintln!("保存新笔记失败: {}", e);
            return;
        }

        self.notes.insert(note_id.clone(), new_note.clone());

        self.sidebar.update(cx, |sidebar, _cx| {
            sidebar.update_notes(self.notes.clone());
            sidebar.set_selected(Some(note_id.clone()));
        });

        self.editor.update(cx, |editor, cx| {
            editor.load_note(new_note, cx);
        });

        cx.notify();
    }

    fn delete_note(&mut self, note_id: String, cx: &mut Context<Self>) {
        if let Err(e) = self.storage.delete_note(&note_id) {
            eprintln!("删除笔记失败: {}", e);
            return;
        }

        self.notes.retain(|k, _| k != &note_id);

        self.sidebar.update(cx, |sidebar, _cx| {
            sidebar.update_notes(self.notes.clone());
            sidebar.set_editing(None);
            sidebar.set_selected(None);
        });

        self.editor.update(cx, |editor, _cx| {
            editor.clear();
        });

        cx.notify();
    }

    fn rename_note(&mut self, note_id: String, new_title: String, cx: &mut Context<Self>) {
        let note_exists = self.notes.contains_key(&note_id);

        if note_exists {
            let note_clone = self.notes.get(&note_id).cloned().unwrap();
            if let Some(note) = self.notes.get_mut(&note_id) {
                note.title = new_title;
                note.updated_at = chrono::Local::now();

                if let Err(e) = self.storage.save_note(note) {
                    eprintln!("重命名笔记失败: {}", e);
                    return;
                }
            }

            self.sidebar.update(cx, |sidebar, _cx| {
                sidebar.update_notes(self.notes.clone());
                sidebar.set_editing(None);
            });

            self.editor.update(cx, |editor, cx| {
                editor.load_note(note_clone, cx);
            });

            cx.notify();
        }
    }

    fn select_note(&mut self, note_id: String, cx: &mut Context<Self>) {
        if let Some(note) = self.notes.get(&note_id) {
            self.editor.update(cx, |editor, cx| {
                editor.load_note(note.clone(), cx);
            });
            self.sidebar.update(cx, |sidebar, _cx| {
                sidebar.set_selected(Some(note_id.clone()));
            });
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
