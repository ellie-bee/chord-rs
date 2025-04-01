use core::fmt;
use std::{collections::HashMap, fs::File, io::Read};

use anyhow::Result;
use serde_json::Value;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct StenoChord {
    pub hash: bool,
    pub sstart: bool,
    pub tstart: bool,
    pub kstart: bool,
    pub pstart: bool,
    pub wstart: bool,
    pub hstart: bool,
    pub rstart: bool,
    pub a: bool,
    pub o: bool,
    pub star: bool,
    pub e: bool,
    pub u: bool,
    pub fend: bool,
    pub rend: bool,
    pub pend: bool,
    pub bend: bool,
    pub lend: bool,
    pub gend: bool,
    pub tend: bool,
    pub send: bool,
    pub dend: bool,
    pub zend: bool,
    pub special_top: bool,
    pub special_bottom: bool,
}

pub fn show_chord_path(chord_path: &[StenoChord]) -> String {
    match chord_path {
        [] => todo!(),
        [c] => return remove_spaces(&c.to_tape()),
        [c, cs @ ..] => return format!("{}/{}", remove_spaces(&c.to_tape()), show_chord_path(cs)),
    }
}

fn remove_spaces(input: &str) -> String {
    input.chars().filter(|&c| c != ' ').collect()
}

pub fn to_steno_chord(raw: &[u8; 6]) -> StenoChord {
    let check_bit = |byte: usize, bit: usize| raw[byte] & (1 << bit) != 0;

    StenoChord {
        special_top: check_bit(2, 1),
        sstart: check_bit(1, 6) || check_bit(1, 5),
        tstart: check_bit(1, 4),
        pstart: check_bit(1, 2),
        hstart: check_bit(1, 0),
        special_bottom: check_bit(2, 0),
        kstart: check_bit(1, 3),
        wstart: check_bit(1, 1),
        rstart: check_bit(2, 6),
        star: check_bit(2, 3) || check_bit(2, 2) || check_bit(3, 5) || check_bit(3, 4),
        hash: check_bit(0, 5) || check_bit(0, 4),
        a: check_bit(2, 5),
        o: check_bit(2, 4),
        fend: check_bit(3, 1),
        pend: check_bit(4, 6),
        lend: check_bit(4, 4),
        tend: check_bit(4, 2),
        dend: check_bit(4, 0),
        rend: check_bit(3, 0),
        bend: check_bit(4, 5),
        gend: check_bit(4, 3),
        send: check_bit(4, 1),
        zend: check_bit(5, 0),
        e: check_bit(3, 3),
        u: check_bit(3, 2),
    }
}

impl StenoChord {
    pub fn to_tape(self) -> String {
        let keys = [
            (self.hash, "#"),
            (self.sstart, "S"),
            (self.tstart, "T"),
            (self.kstart, "K"),
            (self.pstart, "P"),
            (self.wstart, "W"),
            (self.hstart, "H"),
            (self.rstart, "R"),
            (self.a, "A"),
            (self.o, "O"),
            (self.star, "*"),
            (self.e, "E"),
            (self.u, "U"),
            (self.fend, "F"),
            (self.rend, "R"),
            (self.pend, "P"),
            (self.bend, "B"),
            (self.lend, "L"),
            (self.gend, "G"),
            (self.tend, "T"),
            (self.send, "S"),
            (self.dend, "D"),
            (self.zend, "Z"),
            (self.special_top, "^"),
            (self.special_bottom, "_"),
        ];

        keys.iter()
            .map(|&(pressed, key)| if pressed { key } else { " " })
            .collect::<String>()
    }

    pub fn from_str(chord: &str) -> Option<Self> {
        let mut steno_chord = StenoChord {
            hash: false,
            sstart: false,
            tstart: false,
            kstart: false,
            pstart: false,
            wstart: false,
            hstart: false,
            rstart: false,
            a: false,
            o: false,
            star: false,
            e: false,
            u: false,
            fend: false,
            rend: false,
            pend: false,
            bend: false,
            lend: false,
            gend: false,
            tend: false,
            send: false,
            dend: false,
            zend: false,
            special_top: false,
            special_bottom: false,
        };

        let mut right_hand = false;
        for c in chord.chars() {
            match c {
                '#' => steno_chord.hash = true,
                '*' => {
                    steno_chord.star = true;
                    right_hand = true;
                }
                'G' => {
                    steno_chord.gend = true;
                    right_hand = true;
                }
                'S' => {
                    if right_hand {
                        steno_chord.send = true;
                    } else {
                        steno_chord.sstart = true;
                    }
                }
                'T' => {
                    if right_hand {
                        steno_chord.tend = true;
                    } else {
                        steno_chord.tstart = true;
                    }
                }
                'K' => {
                    if right_hand {
                        steno_chord.gend = true;
                    } else {
                        steno_chord.kstart = true;
                    }
                }
                'P' => {
                    if right_hand {
                        steno_chord.pend = true;
                    } else {
                        steno_chord.pstart = true;
                    }
                }
                'W' => steno_chord.wstart = true,
                'H' => steno_chord.hstart = true,
                'R' => {
                    if right_hand {
                        steno_chord.rend = true;
                    } else {
                        steno_chord.rstart = true;
                    }
                }
                'A' => steno_chord.a = true,
                'O' => steno_chord.o = true,
                'E' => steno_chord.e = true,
                'U' => steno_chord.u = true,
                'F' => steno_chord.fend = true,
                'B' => steno_chord.bend = true,
                'L' => steno_chord.lend = true,
                'D' => steno_chord.dend = true,
                'Z' => steno_chord.zend = true,
                '-' => right_hand = true,
                '1' => {
                    steno_chord.hash = true;
                    steno_chord.sstart = true
                }
                '2' => {
                    steno_chord.hash = true;
                    steno_chord.tstart = true
                }
                '3' => {
                    steno_chord.hash = true;
                    steno_chord.pstart = true
                }
                '4' => {
                    steno_chord.hash = true;
                    steno_chord.hstart = true
                }
                '5' => {
                    steno_chord.hash = true;
                    steno_chord.a = true
                }
                '0' => {
                    steno_chord.hash = true;
                    steno_chord.o = true
                }
                '6' => {
                    steno_chord.hash = true;
                    steno_chord.fend = true
                }
                '7' => {
                    steno_chord.hash = true;
                    steno_chord.pend = true
                }
                '8' => {
                    steno_chord.hash = true;
                    steno_chord.lend = true
                }
                '9' => {
                    steno_chord.hash = true;
                    steno_chord.tend = true
                }
                _ => {}
            }
        }

        Some(steno_chord)
    }
}

#[derive(Clone)]
pub enum ChordAction {
    Text(String),
}

pub struct ChordTrie {
    children: HashMap<StenoChord, ChordTrie>,
    action: Option<ChordAction>,
}

impl ChordTrie {
    pub fn new() -> ChordTrie {
        ChordTrie {
            children: HashMap::new(),
            action: None,
        }
    }

    pub fn from_dict(dict_path: &str) -> Result<ChordTrie> {
        let mut trie = ChordTrie::new();
        let mut file: File = File::open(dict_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let raw_dict: Value = serde_json::from_str(&contents)?;
        let raw_dict = raw_dict
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("Expected JSON object"))?;

        for (k, v) in raw_dict {
            if let Some(chord_path) = k
                .split("/")
                .map(StenoChord::from_str)
                .collect::<Option<Vec<StenoChord>>>()
            {
                trie.insert(
                    chord_path.as_slice(),
                    ChordAction::Text(v.to_string().replace("\"", "")),
                )?;
            }
        }

        Ok(trie)
    }

    fn insert(&mut self, chord_path: &[StenoChord], action: ChordAction) -> Result<()> {
        match chord_path {
            [] => {
                self.action = Some(action);
                Ok(())
            }

            [head, rest @ ..] => match self.children.get_mut(head) {
                Some(c) => {
                    _ = c.insert(rest, action);
                    Ok(())
                }
                None => {
                    let mut new_trie = ChordTrie::new();
                    _ = new_trie.insert(rest, action);
                    self.children.insert(*head, new_trie);
                    Ok(())
                }
            },
        }
    }

    pub fn lookup(&self, chord_path: &[StenoChord]) -> Option<ChordAction> {
        match chord_path {
            [] => None,
            [head] => match self.children.get(head) {
                Some(s) => s.action.clone(),
                None => None,
            },
            [head, rest @ ..] => match self.children.get(head) {
                Some(s) => s.lookup(rest),
                None => None,
            },
        }
    }

    pub fn has_prefix(&self, chord_path: &[StenoChord]) -> bool {
        let mut current = self;
        for c in chord_path {
            match current.children.get(c) {
                Some(child) => {
                    current = child;
                }
                None => {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_complete_word(&self, chord_path: &[StenoChord]) -> bool {
        self.lookup(chord_path).is_some()
    }
}
