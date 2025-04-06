use anyhow::Result;

mod action;
mod cre;
mod dictionary;
mod engine;
mod geminipr;
mod machine;

use dictionary::Dictionary;
use engine::Engine;
use geminipr::GeminiPR;

fn main() -> Result<()> {
    let dictionary = Dictionary::load_from_json("dict/main.json")?;
    // dictionary.print_dict();

    let mut engine = Engine::new();

    engine.include(dictionary);
    engine.connect(GeminiPR::new("/dev/ttyACM0")?)?;

    engine.run();

    engine.disconnect();

    Ok(())
}
