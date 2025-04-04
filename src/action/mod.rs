use anyhow::Result;

#[derive(Debug)]
pub enum Action {
    Text(String),
}

impl Action {
    pub(crate) fn from_str(v: &serde_json::Value) -> Result<Action> {
        if let Some(text) = v.as_str() {
            Ok(Action::Text(text.to_string()))
        } else {
            Err(anyhow::anyhow!("Malformed entry: {:?}", v))
        }
    }
}
