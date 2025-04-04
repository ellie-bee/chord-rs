use anyhow::Result;

mod action;
mod bbsteno;
mod cre;
mod dictionary;
mod engine;
mod machine;

use bbsteno::BBSteno;
use dictionary::Dictionary;
use engine::Engine;

fn main() -> Result<()> {
    let dictionary = Dictionary::load_from_json("dict/main.json")?;
    dictionary.print_dict();

    let mut engine = Engine::new();

    engine.include(dictionary);
    engine.connect(BBSteno::new("/dev/ttyACM0")?)?;

    engine.run();

    engine.disconnect();

    Ok(())
}
