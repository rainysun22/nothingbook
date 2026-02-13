use crate::{
    note::Note,
    note_list::NoteList,
    views::{
        editor::EditorView,
        sidebar::{SidebarEvent, SidebarView},
    },
};
use gpui::*;
use gpui_component::h_flex;

pub struct AppView {
    sidebar: Entity<SidebarView>,
    editor: Entity<EditorView>,
    notes: Entity<NoteList>,
}

impl AppView {
    pub fn new(cx: &mut Context<Self>) -> anyhow::Result<Self> {
        let notes = cx.new(|cx| NoteList::new(cx));
        let sidebar = cx.new(|_cx| SidebarView::new(notes.clone()));
        let editor = cx.new(|cx| EditorView::new(cx));

        let app = Self {
            sidebar,
            editor,
            notes,
        };

        cx.subscribe(
            &app.sidebar,
            |this: &mut AppView, _, event: &SidebarEvent, cx| {
                this.handle_sidebar_event(event, cx);
            },
        )
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

        if let Err(e) = self.notes.update(cx, |notes, _cx| notes.add(note.clone())) {
            eprintln!("保存新笔记失败: {}", e);
            return;
        }

        self.sidebar.update(cx, |sidebar, _cx| {
            sidebar.set_selected(Some(id));
        });

        self.editor.update(cx, |editor, cx| {
            editor.load_note(&note, cx);
        });

        cx.notify();
    }

    fn delete_note(&mut self, note_id: u128, cx: &mut Context<Self>) {
        if let Err(e) = self.notes.update(cx, |notes, _cx| notes.remove(note_id)) {
            eprintln!("删除笔记失败: {}", e);
            return;
        }

        self.sidebar.update(cx, |sidebar, _cx| {
            sidebar.set_selected(None);
        });

        self.editor.update(cx, |editor, _cx| {
            editor.clear();
        });

        cx.notify();
    }

    fn select_note(&mut self, note_id: u128, cx: &mut Context<Self>) {
        let note_clone = self
            .notes
            .update(cx, |notes, _cx| notes.get(note_id).cloned());

        if let Some(note) = note_clone {
            self.editor.update(cx, |editor, cx| {
                editor.load_note(&note, cx);
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
