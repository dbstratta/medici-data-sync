use serde::Deserialize;
use uuid::Uuid;

use crate::raw_data::{RawOptionData, RawQuestionData};

#[derive(Deserialize, Debug)]
#[serde(from = "RawQuestionData")]
pub struct QuestionData {
    pub id: Uuid,

    pub text: String,
    pub options: Vec<OptionData>,
    pub evaluation: String,

    pub hash: Vec<u8>,
}

impl QuestionData {
    fn new(id: Uuid, text: String, options: Vec<OptionData>, evaluation: String) -> Self {
        let hash = "".into();

        Self {
            id,
            text,
            options,
            evaluation,
            hash,
        }
    }
}

impl From<RawQuestionData> for QuestionData {
    fn from(raw: RawQuestionData) -> Self {
        let options = raw.options.into_iter().map(Into::into).collect();

        Self::new(
            raw.id.unwrap_or_else(|| Uuid::new_v4()),
            raw.text,
            options,
            raw.evaluation,
        )
    }
}

#[derive(Deserialize, Debug)]
pub struct OptionData {
    pub id: Uuid,

    pub text: String,
    pub correct: bool,

    pub hash: Vec<u8>,
}

impl OptionData {
    fn new(id: Uuid, text: String, correct: bool) -> Self {
        let hash = "".into();

        Self {
            id,
            text,
            correct,
            hash,
        }
    }
}

impl From<RawOptionData> for OptionData {
    fn from(raw: RawOptionData) -> Self {
        Self::new(
            raw.id.unwrap_or_else(|| Uuid::new_v4()),
            raw.text,
            raw.correct.unwrap_or(false),
        )
    }
}
