pub mod note;
pub mod score;

use rodio::{default_output_device, source::SineWave, Sink, Source};
use score::Score;
use std::time::Duration;

pub fn play_music(input: Vec<u8>) {
    let device = default_output_device().unwrap();
    let sink = Sink::new(&device);
    let music_score = Score::new(input);
    for note in music_score.notes {
        let source = SineWave::new(note.freq);
        let source = source.take_duration(Duration::from_millis(note.duration / 5));
        sink.append(source);
    }
    sink.sleep_until_end();
}
