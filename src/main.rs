use anyhow::Result;

mod chording_engine;

use chording_engine::dictionary::Dictionary;
use chording_engine::{engine::Engine, machines::geminipr::GeminiPR};

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
