use crate::cre::stroke::*;
use anyhow::Result;
use core::fmt;

#[derive(Hash, PartialEq, Eq)]
pub struct Chord(Vec<Stroke>);

impl Chord {
    pub fn from_str(k: &str) -> Result<Chord> {
        let strokes: Result<Vec<Stroke>> = k.split("/").map(Stroke::from_str).collect();

        Ok(Chord(strokes?))
    }
}

impl fmt::Debug for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.0.iter();
        while let Some(s1) = iter.next() {
            write!(f, "{s1:?}")?;

            if let Some(s2) = iter.next() {
                f.write_str("/")?;
                write!(f, "{s2:?}")?;
            }
        }
        Ok(())
    }
}
