use crate::{
    note::Note,
    storage::Storage,
    views::{
        editor::EditorView,
        sidebar::{SidebarEvent, SidebarView},
    },
};
use gpui::*;
use gpui_component::h_flex;
use std::collections::HashMap;

pub struct AppView {
    sidebar: Entity<SidebarView>,
    editor: Entity<EditorView>,
    storage: Storage,
    notes: HashMap<u128, Note>,
}

impl AppView {
    pub fn new(cx: &mut Context<Self>) -> anyhow::Result<Self> {
        let storage = Storage::new()?;
        let mut notes: HashMap<u128, Note> = HashMap::new();
        storage.load_all_notes(&mut notes)?;

        let sidebar = cx.new(|_cx| {
            SidebarView::new()
        });
        let editor = cx.new(|cx| EditorView::new(cx));

        let app = Self {
            sidebar,
            editor,
            storage,
            notes,
        };

        if let Some(note_id) = app.notes.keys().next().copied() {
            if let Some(note) = app.notes.get(&note_id) {
                app.editor.update(cx, |editor, cx| {
                    editor.load_note(note, cx);
                });
                app.sidebar.update(cx, |sidebar, _cx| {
                    sidebar.set_selected(Some(note_id));
                });
            }
        }

        cx.subscribe(&app.sidebar, |this: &mut AppView, _, event: &SidebarEvent, cx| {
            this.handle_sidebar_event(event, cx);
        })
        .detach();  

        // TODO：订阅edit事件

        Ok(app)
    }

    fn handle_sidebar_event(&mut self, event: &SidebarEvent, cx: &mut Context<Self>) {
        match event {
            SidebarEvent::CreateNote => self.create_note(cx),
            SidebarEvent::DeleteNote(note_id) => self.delete_note(*note_id, cx),
            SidebarEvent::SelectNote(note_id) => self.select_note(*note_id, cx),
        }
    }

    fn create_note(&mut self, cx: &mut Context<Self>) {
        let note = Note::new();
        let id = note.id;

        if let Err(e) = self.storage.save_note(&note) {
            eprintln!("保存新笔记失败: {}", e);
            return;
        }

        self.sidebar.update(cx, |sidebar, _cx| {
            sidebar.set_selected(Some(id));
        });

        self.editor.update(cx, |editor, cx| {
            editor.load_note(&note, cx);
        });

        self.notes.insert(id, note);
        cx.notify();
    }

    fn delete_note(&mut self, note_id: u128, cx: &mut Context<Self>) {
        if let Err(e) = self.storage.delete_note(note_id) {
            eprintln!("删除笔记失败: {}", e);
            return;
        }

        self.notes.retain(|k, _| k != &note_id);

        self.sidebar.update(cx, |sidebar, _cx| {
            sidebar.set_selected(None);
        });

        self.editor.update(cx, |editor, _cx| {
            editor.clear();
        });

        cx.notify();
    }

    fn select_note(&mut self, note_id: u128, cx: &mut Context<Self>) {
        if let Some(note) = self.notes.get(&note_id) {
            self.editor.update(cx, |editor, cx| {
                editor.load_note(note, cx);
            });
            self.sidebar.update(cx, |sidebar, _cx| {
                sidebar.set_selected(Some(note_id));
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
