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

            self.new_stroke(stroke);

            println!("{}", stroke.as_tape());
        }
    }
    pub(crate) fn disconnect(&mut self) {
        self.machine.as_mut().map(|m| m.disconnect());
        self.machine = None;
    }

    fn translate_strokes(&self) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();
        let strokes: Vec<_> = self.check_strokes.iter().cloned().collect();

        let mut start = 0;
        while start < strokes.len() {
            let mut best_match: Option<(usize, &Action)> = None;

            for end in (start + 1)..=strokes.len() {
                let prefix = &strokes[start..end];
                if let Some(action) = self.dictionary.lookup(prefix) {
                    best_match = Some((end, action));
                }
            }

            if let Some((next_start, action)) = best_match {
                actions.push(action.clone());
                start = next_start;
            } else {
                start += 1;
            }
        }

        actions
    }

    fn new_stroke(&mut self, stroke: Stroke) {
        self.check_strokes.push_back(stroke);
        if self.check_strokes.len() > self.dictionary.longest_key_len() {
            self.check_strokes.pop_front();
        }

        let actions = self.translate_strokes();

        self.action_differences(&actions);

        self.action_history.extend_from_slice(&actions.clone());
    }

    fn action_differences(&self, actions: &[Action]) {
        let mut updates = Vec::new();
        let mut index = 0;

        while index < self.action_history.len() && index < actions.len() {
            if self.action_history[index] != actions[index] {
                if let Action::Text(t) = &self.action_history[index] {
                    updates.push(Action::Undo(t.len()));
                }
                updates.push(actions[index].clone());
            }

            index += 1;
        }

        while index < actions.len() {
            updates.push(actions[index].clone());
            index += 1;
        }

        updates.iter().for_each(|action| action.execute_action());
    }
}
