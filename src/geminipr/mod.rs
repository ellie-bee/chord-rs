use crate::{cre::Stroke, machine::Machine};
use anyhow::Result;
use serial2::SerialPort;

pub struct GeminiPR {
    tty_path: String,
    port: Option<SerialPort>,
}

impl GeminiPR {
    pub(crate) fn new(tty_path: &str) -> Result<Self> {
        Ok(Self {
            tty_path: tty_path.into(),
            port: None,
        })
    }
}

impl Machine for GeminiPR {
    fn connect(&mut self) -> Result<()> {
        let port = SerialPort::open(&self.tty_path, 115200)?;
        self.port = Some(port);
        Ok(())
    }

    fn disconnect(&mut self) -> Result<()> {
        self.port = None;
        Ok(())
    }

    fn get_stroke(&mut self) -> Result<Stroke> {
        let mut buffer = [0; 6];
        match &self.port {
            Some(port) => {
                port.read(&mut buffer)?;
                Ok(to_stroke(&buffer))
            }
            None => unreachable!("Machine is not connected"),
        }
    }
}

fn to_stroke(buffer: &[u8; 6]) -> Stroke {
    let check_bit = |byte: &usize, bit: &usize| buffer[*byte] & (1 << bit) != 0;

    [
        (1, 6, Stroke::START_S),
        (1, 5, Stroke::START_S),
        (1, 4, Stroke::START_T),
        (1, 3, Stroke::START_K),
        (1, 2, Stroke::START_P),
        (1, 1, Stroke::START_W),
        (1, 0, Stroke::START_H),
        (2, 6, Stroke::START_R),
        (2, 3, Stroke::STAR),
        (2, 2, Stroke::STAR),
        (3, 5, Stroke::STAR),
        (3, 4, Stroke::STAR),
        (0, 5, Stroke::HASH),
        (0, 4, Stroke::HASH),
        (2, 5, Stroke::START_A),
        (2, 4, Stroke::START_O),
        (3, 1, Stroke::END_F),
        (4, 6, Stroke::END_P),
        (4, 4, Stroke::END_L),
        (4, 2, Stroke::END_T),
        (4, 0, Stroke::END_D),
        (3, 0, Stroke::END_R),
        (4, 5, Stroke::END_B),
        (4, 3, Stroke::END_G),
        (4, 1, Stroke::END_S),
        (5, 0, Stroke::END_Z),
        (3, 3, Stroke::END_E),
        (3, 2, Stroke::END_U),
    ]
    .into_iter()
    .filter(|(byte, bit, _)| check_bit(byte, bit))
    .map(|x| x.2)
    .reduce(|x, y| x | y)
    .unwrap_or(Stroke::empty())
}
