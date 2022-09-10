use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{CourseData, QuestionData, QuestionOptionData};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncData {
    pub courses_to_sync: Vec<CourseData>,
    pub courses_to_delete: Vec<String>,

    pub questions_to_sync: Vec<QuestionData>,
    pub questions_to_delete: Vec<Uuid>,

    pub question_options_to_sync: Vec<QuestionOptionData>,
    pub question_options_to_delete: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncMetadata {
    pub courses_metadata: HashMap<String, String>,
    pub questions_metadata: HashMap<Uuid, String>,
    pub question_options_metadata: HashMap<Uuid, String>,
    pub course_evaluations: HashMap<String, Vec<String>>,
}
