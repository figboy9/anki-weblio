use std::env;

use anki::add_note;
mod anki;
use anyhow::{bail, Ok, Result};
use weblio::scrape_translation;
mod weblio;

struct Traslation {
    word: String,
    explanation: String,
    audio_url: Option<String>,
}
fn main() -> Result<()> {
    let mut word = String::new();
    if let Some(w) = env::args().nth(1) {
        word = w;
    }

    if word.is_empty() {
        bail!("word required");
    }
    let translation = scrape_translation(word)?;
    add_note(&translation)?;

    println!("{}:\n{}", translation.word, translation.explanation);
    Ok(())
}
