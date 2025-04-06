use std::collections::VecDeque;

use crate::{cre::Stroke, dictionary::Dictionary, machine::Machine};
use anyhow::Result;

pub struct Engine<M: Machine> {
    dictionary: Dictionary,
    machine: Option<M>,
    previous_strokes: Vec<Stroke>,
    check_strokes: VecDeque<Stroke>,
}

impl<M> Engine<M>
where
    M: Machine,
{
    pub fn new() -> Self {
        Engine {
            dictionary: Dictionary::new(),
            machine: None,
            previous_strokes: Vec::new(),
            check_strokes: VecDeque::new(),
        }
    }

    pub(crate) fn connect(&mut self, mut machine: M) -> Result<()> {
        machine.connect()?;
        self.machine = Some(machine);
        Ok(())
    }

    pub(crate) fn include(&mut self, dictionary: Dictionary) {
        self.dictionary.0.extend(dictionary.0);
    }

    pub(crate) fn run(&mut self) {
        loop {
            let Some(Ok(stroke)) = self.machine.as_mut().map(|m| m.get_stroke()) else {
                continue;
            };

            self.previous_strokes.push(stroke);
            self.check_strokes.push(stroke);

            let Some(action) = self.dictionary.lookup(Chord::new(check_strokes));

            println!("{}", stroke.as_tape());
        }
    }
    pub(crate) fn disconnect(&mut self) {
        self.machine.as_mut().map(|m| m.disconnect());
        self.machine = None;
    }
}
