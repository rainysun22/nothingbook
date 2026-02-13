use crate::note::Note;
use crate::storage::Storage;
use anyhow::Result;

use gpui::Context;
use std::collections::HashMap;

pub struct NoteList {
    notes: HashMap<u128, Note>,
    storage: Storage,
}

impl NoteList {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        let mut notes: HashMap<u128, Note> = HashMap::new();
        let storage = Storage::new().unwrap();
        let _ = storage.load_all_notes(&mut notes);
        NoteList { notes, storage }
    }

    pub fn add(&mut self, note: Note) -> Result<()> {
        self.storage.save_note(&note)?;
        self.notes.insert(note.id, note);
        Ok(())
    }

    pub fn remove(&mut self, id: u128) -> Result<()> {
        self.storage.delete_note(id)?;
        self.notes.remove(&id);
        Ok(())
    }

    pub fn get(&self, id: u128) -> Option<&Note> {
        self.notes.get(&id)
    }

    pub fn get_all(&self) -> Vec<&Note> {
        self.notes.values().collect()
    }
}
