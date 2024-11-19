use anyhow::{bail, Ok, Result};
use scraper::Html;

use crate::Traslation;

pub fn scrape_translation(word: String) -> Result<Traslation> {
    let body = reqwest::blocking::get(format!("https://ejje.weblio.jp/content/{word}"))?;
    let html = Html::parse_document(std::str::from_utf8(&body.bytes()?)?);

    let explanation = find_explanation(&html)?;
    let audio_url = find_audio_url(&html);

    Ok(Traslation {
        word,
        explanation,
        audio_url,
    })
}

fn find_explanation(html: &Html) -> Result<String> {
    let selector = scraper::Selector::parse("span.content-explanation").unwrap();

    if let Some(s) = html.select(&selector).next() {
        let text = s.text().collect::<Vec<_>>();
        return Ok(text[0].trim().to_string());
    }

    bail!("failed get explanation");
}
fn find_audio_url(html: &Html) -> Option<String> {
    let mut audio_url = None;
    let selector = scraper::Selector::parse("#summary audio source").unwrap();

    if let Some(s) = html.select(&selector).next() {
        let url = s.attr("src").unwrap().to_string();
        audio_url = Some(url);
    }

    audio_url
}
