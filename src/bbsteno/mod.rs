// implementing the specifics for the bbsteno machine

use anyhow::Result;
use serial2::SerialPort;

use crate::machine::Machine;

pub struct BBSteno {
    tty_path: String,
    port: Option<SerialPort>,
}

impl BBSteno {
    pub(crate) fn new(tty_path: &str) -> Result<Self> {
        Ok(Self {
            tty_path: tty_path.into(),
            port: None,
        })
    }
}

impl Machine for BBSteno {
    fn connect(&mut self) -> Result<()> {
        let port = SerialPort::open(&self.tty_path, 115200)?;
        self.port = Some(port);
        Ok(())
    }

    fn disconnect(&mut self) -> Result<()> {
        self.port = None;
        Ok(())
    }
}
