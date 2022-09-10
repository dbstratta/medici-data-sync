use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::{QuestionData, QuestionOptionData};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawQuestionData {
    pub id: Option<Uuid>,

    pub text: String,
    pub options: Vec<RawQuestionOptionData>,
    pub evaluation: String,
}

impl From<QuestionData> for RawQuestionData {
    fn from(data: QuestionData) -> Self {
        let raw_question_options = data.question_options.into_iter().map(Into::into).collect();

        Self {
            id: Some(data.id),
            text: data.text,
            options: raw_question_options,
            evaluation: data.evaluation,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawQuestionOptionData {
    pub id: Option<Uuid>,

    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

impl From<QuestionOptionData> for RawQuestionOptionData {
    fn from(data: QuestionOptionData) -> Self {
        Self {
            id: Some(data.id),
            text: data.text,
            correct: if data.correct {
                Some(data.correct)
            } else {
                None
            },
            explanation: data.explanation,
        }
    }
}
