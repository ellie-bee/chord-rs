use anyhow::Result;
use serial2::SerialPort;
mod types;
use types::{ChordAction, ChordTrie, StenoChord, show_chord_path, to_steno_chord};

fn get_chord(port: &SerialPort) -> Result<StenoChord> {
    let mut buffer = [0; 6];
    port.read(&mut buffer)?;
    Ok(to_steno_chord(&buffer))
}

fn process(chord_trie: &ChordTrie, port: &mut SerialPort) {
    let mut chord_path: Vec<StenoChord> = Vec::new();
    let mut candidate: Option<Vec<StenoChord>> = None;

    loop {
        if let Ok(chord) = get_chord(port) {
            chord_path.push(chord);
            print!("{}", chord.to_tape());
            if chord_trie.has_prefix(&chord_path) {
                if chord_trie.is_complete_word(&chord_path) {
                    candidate = Some(chord_path.clone());
                }
            } else if let Some(valid_path) = candidate.clone() {
                if let Some(ChordAction::Text(word)) = chord_trie.lookup(&valid_path) {
                    println!("{}", word);
                    chord_path = vec![chord];
                    candidate = if chord_trie.is_complete_word(&chord_path) {
                        Some(chord_path.clone())
                    } else {
                        None
                    };
                }
            } else {
                println!("???");
                chord_path.clear();
                candidate = None;
            }
        }
    }
}

fn main() -> Result<()> {
    let chords = ChordTrie::from_dict("dict/main.json").expect("Unable to find dictionary");
    let mut port = SerialPort::open("/dev/ttyACM0", 115200).expect("Unable to open serial port");

    // print_dict(&chords);

    process(&chords, &mut port);
    Ok(())
}

fn print_dict(chords: &ChordTrie) {
    todo!()
}
