use std::collections::VecDeque;

use crate::chording_engine::{
    action::Action, cre::Stroke, dictionary::Dictionary, machines::Machine,
};
use anyhow::Result;

pub struct Engine<M: Machine> {
    dictionary: Dictionary,
    machine: Option<M>,
    check_strokes: VecDeque<Stroke>,
    action_history: Vec<Action>,
}

impl<M> Engine<M>
where
    M: Machine,
{
    pub fn new() -> Self {
        Engine {
            dictionary: Dictionary::new(),
            machine: None,
            check_strokes: VecDeque::new(),
            action_history: Vec::new(),
        }
    }

    pub(crate) fn connect(&mut self, mut machine: M) -> Result<()> {
        machine.connect()?;
        self.machine = Some(machine);
        Ok(())
    }

    pub(crate) fn include(&mut self, dictionary: Dictionary) {
        self.dictionary.extend(dictionary);
    }

    pub(crate) fn run(&mut self) {
        loop {
            let Some(Ok(stroke)) = self.machine.as_mut().map(|m| m.get_stroke()) else {
                continue;
            };

            self.check_strokes.push_back(stroke);
            if self.check_strokes.len() > self.dictionary.longest_key_len() {
                self.check_strokes.pop_front();
            }

            let mut actions: Vec<Action> = Vec::new();
            let strokes: Vec<_> = self.check_strokes.iter().cloned().collect();

            let mut i = 0;
            while i < strokes.len() {
                let mut best_match: Option<(usize, &Action)> = None;

                // find the longest prefix of strokes that is a valid entry in the dictionary
                for j in (i + 1)..=strokes.len() {
                    let prefix = &strokes[i..j];
                    if let Some(action) = self.dictionary.lookup(prefix) {
                        best_match = Some((j, action));
                    }
                }

                if let Some((next_i, action)) = best_match {
                    actions.push(action.clone());
                    i = next_i;
                } else {
                    i += 1;
                }
            }

            println!("{} | {:?}", stroke.as_tape(), actions);
        }
    }
    pub(crate) fn disconnect(&mut self) {
        self.machine.as_mut().map(|m| m.disconnect());
        self.machine = None;
    }
}
