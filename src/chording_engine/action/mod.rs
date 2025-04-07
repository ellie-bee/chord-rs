use anyhow::Result;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Text(String),
    Undo(usize),
}

impl Action {
    pub(crate) fn from_str(v: &serde_json::Value) -> Result<Action> {
        if let Some(text) = v.as_str() {
            Ok(Action::Text(text.to_string()))
        } else {
            Err(anyhow::anyhow!("Malformed entry: {:?}", v))
        }
    }

    pub(crate) fn execute_action(&self) {
        let mut enigo =
            Enigo::new(&Settings::default()).expect("Unwable to mount virtual keyboard");
        match self {
            Action::Text(t) => {
                enigo.text(t).expect("Unable to send text");
            }
            Action::Undo(n) => {
                (1..*n).for_each(|_| {
                    enigo
                        .key(Key::Backspace, Direction::Press)
                        .expect("Unable to use keys?");
                    enigo
                        .key(Key::Backspace, Direction::Release)
                        .expect("Unable to use keys?");
                });
            }
        }
    }
}
