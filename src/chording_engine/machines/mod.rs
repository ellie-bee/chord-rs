use anyhow::Result;

use crate::chording_engine::cre::Stroke;

pub(crate) mod geminipr;

pub trait Machine {
    fn connect(&mut self) -> Result<()>;
    fn disconnect(&mut self) -> Result<()>;
    fn get_stroke(&mut self) -> Result<Stroke>;
}
