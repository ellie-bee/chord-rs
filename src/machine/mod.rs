use anyhow::Result;

pub trait Machine {
    fn connect(&mut self) -> Result<()>;
    fn disconnect(&mut self) -> Result<()>;
}
