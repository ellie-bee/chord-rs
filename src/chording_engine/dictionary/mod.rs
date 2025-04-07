use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

use crate::chording_engine::{
    action::Action,
    cre::{Chord, Stroke},
};
use anyhow::Result;
use serde_json::Value;

pub struct Dictionary {
    internal_dict: HashMap<Chord, Action>,
    longest_key_len: Option<usize>,
}

impl Dictionary {
    pub(crate) fn load_from_json(dictionary_path: &str) -> Result<Self> {
        let mut dict = HashMap::new();

        let mut file: File = File::open(dictionary_path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let raw_dict: Value = serde_json::from_str(&contents)?;
        let raw_dict = raw_dict
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("Expected JSON object"))?;

        for (k, v) in raw_dict {
            match (Chord::from_str(k), Action::from_str(v)) {
                (Ok(chord), Ok(action)) => _ = dict.insert(chord, action),
                _ => continue,
            }
        }

        Ok(Dictionary {
            internal_dict: dict,
            longest_key_len: None,
        })
    }

    pub(crate) fn print_dict(&self) {
        for (k, v) in self.internal_dict.iter() {
            println!("{:?}: {:?}", k, v);
        }
    }

    pub(crate) fn new() -> Self {
        Dictionary {
            internal_dict: HashMap::new(),
            longest_key_len: None,
        }
    }

    pub fn lookup(&self, strokes: &[Stroke]) -> Option<&Action> {
        let chord = Chord::new(strokes.to_vec());
        self.internal_dict.get(&chord)
    }

    pub(crate) fn extend(&mut self, dictionary: Dictionary) {
        self.update_longest_key_len();
        self.internal_dict.extend(dictionary.internal_dict)
    }

    fn update_longest_key_len(&mut self) -> usize {
        if let Some(longest) = self.internal_dict.keys().reduce(|prev, next| {
            if prev.length() > next.length() {
                prev
            } else {
                next
            }
        }) {
            self.longest_key_len = Some(longest.length());
            longest.length()
        } else {
            0
        }
    }

    pub(crate) fn longest_key_len(&mut self) -> usize {
        match self.longest_key_len {
            Some(s) => s,
            None => self.update_longest_key_len(),
        }
    }
}
