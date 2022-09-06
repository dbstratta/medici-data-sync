use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawQuestionData {
    pub id: Option<Uuid>,

    pub text: String,
    pub options: Vec<RawOptionData>,
    pub evaluation: String,
}

#[derive(Deserialize, Debug)]
pub struct RawOptionData {
    pub id: Option<Uuid>,

    pub text: String,
    pub correct: Option<bool>,
}
