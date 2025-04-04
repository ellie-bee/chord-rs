use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::action::Action;

use crate::cre::Chord;
use anyhow::Result;
use serde_json::Value;

pub struct Dictionary(pub HashMap<Chord, Action>);

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
                _ => print!("Malformed dictionary entry: {:?} {:?}", k, v),
            }
        }

        Ok(Dictionary(dict))
    }

    pub(crate) fn print_dict(&self) {
        for (k, v) in self.0.iter() {
            println!("{:?}: {:?}", k, v);
        }
    }
}
