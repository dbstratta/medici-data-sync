use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{CourseData, OptionData, QuestionData};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncData {
    pub courses_to_sync: Vec<CourseData>,
    pub courses_to_delete: Vec<String>,

    pub questions_to_sync: Vec<QuestionData>,
    pub questions_to_delete: Vec<Uuid>,

    pub options_to_sync: Vec<OptionData>,
    pub options_to_delete: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncMetadata {
    pub courses_metadata: HashMap<String, String>,
    pub questions_metadata: HashMap<Uuid, String>,
    pub options_metadata: HashMap<Uuid, String>,
}
