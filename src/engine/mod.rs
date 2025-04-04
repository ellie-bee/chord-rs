use crate::{dictionary::Dictionary, machine::Machine};
use anyhow::Result;

pub struct Engine<M: Machine> {
    dictionary: Dictionary,
    machine: Option<M>,
}

impl<M> Engine<M>
where
    M: Machine,
{
    pub fn new() -> Self {
        todo!()
    }

    pub(crate) fn connect(&mut self, mut machine: M) -> Result<()> {
        machine.connect()?;
        self.machine = Some(machine);
        Ok(())
    }

    pub(crate) fn include(&mut self, dictionary: Dictionary) {
        self.dictionary.0.extend(dictionary.0);
    }

    pub(crate) fn run(&self) {
        loop {
            todo!()
        }
    }
    pub(crate) fn disconnect(&mut self) {
        self.machine.as_mut().map(|m| m.disconnect());
        self.machine = None;
    }
}
