use anyhow::Result;

use crate::cre::Stroke;

pub trait Machine {
    fn connect(&mut self) -> Result<()>;
    fn disconnect(&mut self) -> Result<()>;
    fn get_stroke(&mut self) -> Result<Stroke>;
}
