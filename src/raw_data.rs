use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::{OptionData, QuestionData};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawQuestionData {
    pub id: Option<Uuid>,

    pub text: String,
    pub options: Vec<RawOptionData>,
    pub evaluation: String,
}

impl From<QuestionData> for RawQuestionData {
    fn from(data: QuestionData) -> Self {
        let raw_options = data.options.into_iter().map(Into::into).collect();

        Self {
            id: Some(data.id),
            text: data.text,
            options: raw_options,
            evaluation: data.evaluation,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawOptionData {
    pub id: Option<Uuid>,

    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct: Option<bool>,
}

impl From<OptionData> for RawOptionData {
    fn from(data: OptionData) -> Self {
        Self {
            id: Some(data.id),
            text: data.text,
            correct: if data.correct {
                Some(data.correct)
            } else {
                None
            },
        }
    }
}
