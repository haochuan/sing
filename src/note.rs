pub struct Note {
    pub freq: u32,
    pub duration: u64,
}

impl Note {
    pub fn new(b: u8) -> Self {
        let map: [u32; 12] = [262, 278, 294, 311, 330, 349, 370, 392, 415, 440, 466, 494];
        let freq = map[(b % 12) as usize];
        let duration = (b % 4 + 1) as u64 * 1000;
        Self { freq, duration }
    }
}

#[cfg(test)]
mod tests {
    use super::Note;
    #[test]
    fn it_works() {
        let text = "hello";
        let bytes = text.as_bytes();
        let target_freq: [u32; 5] = [415, 349, 262, 262, 311];
        let target_duration: [u64; 5] = [1000, 2000, 1000, 1000, 4000];
        let mut index = 0;
        for b in bytes {
            let note = Note::new(*b);
            assert_eq!(note.freq, target_freq[index]);
            assert_eq!(note.duration, target_duration[index]);
            index += 1;
        }
    }
}
