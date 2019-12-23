use crate::note;

pub struct Score {
    pub notes: Vec<note::Note>,
}

impl Score {
    pub fn new(btyes: Vec<u8>) -> Self {
        let mut notes: Vec<note::Note> = vec![];
        for b in btyes {
            let note = note::Note::new(b);
            notes.push(note);
        }
        Self { notes }
    }
}
