use anyhow::{bail, Ok, Result};
use serde::{Deserialize, Serialize};

use crate::Traslation;

pub fn add_note(traslation: &Traslation) -> Result<u64> {
    let client = reqwest::blocking::Client::new();
    let req = build_add_note_request(traslation);

    let res = client.post("http://localhost:8765").json(&req).send()?;
    let res = res.json::<AnkiResponse<AddNoteResult>>()?;
    if let Some(error) = res.error {
        bail!(error)
    }

    Ok(res.result.unwrap().0)
}

fn build_add_note_request(traslation: &Traslation) -> AnkiRequest<AddNote> {
    let mut params = AddNote {
        note: Note {
            deck_name: "Default".to_string(),
            model_name: "Basic".to_string(),
            fields: Fields {
                front: traslation.word.clone(),
                back: traslation.explanation.clone(),
            },
            options: None,
            tags: None,
            audio: None,
            video: None,
            picture: None,
        },
    };
    if let Some(audio_url) = &traslation.audio_url {
        params.note.audio = Some(vec![Audio {
            url: audio_url.to_string(),
            filename: format!("{}.mp3", traslation.word.clone()),
            skip_hash: None,
            fields: vec!["Front".to_string()],
        }]);
        // audio_url
    }

    AnkiRequest::new(params)
}

#[derive(Serialize, Debug)]
struct AnkiRequest<T> {
    action: String,
    version: u8,
    params: Option<T>,
}

impl AnkiRequest<AddNote> {
    fn new(params: AddNote) -> Self {
        Self {
            action: "addNote".to_string(),
            version: 6,
            params: Some(params),
        }
    }
}

#[derive(Deserialize, Debug)]
struct AnkiResponse<T> {
    result: Option<T>,
    error: Option<String>,
}

#[derive(Deserialize, Debug)]
struct AddNoteResult(u64);

#[derive(Debug, Serialize)]
pub struct AddNote {
    pub note: Note,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub deck_name: String,
    pub model_name: String,
    pub fields: Fields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Vec<Audio>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Vec<Video>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Vec<Picture>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    #[serde(rename = "Front")]
    pub front: String,
    #[serde(rename = "Back")]
    pub back: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub allow_duplicate: bool,
    pub duplicate_scope: String,
    pub duplicate_scope_options: DuplicateScopeOptions,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateScopeOptions {
    pub deck_name: String,
    pub check_children: bool,
    pub check_all_models: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Audio {
    pub url: String,
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_hash: Option<String>,
    pub fields: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub url: String,
    pub filename: String,
    pub skip_hash: String,
    pub fields: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Picture {
    pub url: String,
    pub filename: String,
    pub skip_hash: String,
    pub fields: Vec<String>,
}
