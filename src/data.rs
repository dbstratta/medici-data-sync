use std::cmp::Ordering;
use std::fs::DirEntry;
use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;
use uuid::Uuid;

use crate::helpers::{read_dir_entry_data, write_data};
use crate::raw_data::{RawOptionData, RawQuestionData};

#[derive(Deserialize, Clone, Debug)]
pub struct CourseData {
    pub key: String,

    pub questions: Vec<QuestionData>,

    pub hash: String,
}

impl CourseData {
    pub fn new(key: String, questions: Vec<QuestionData>) -> Self {
        let hash = Self::hash_data(&key, &questions[..]);

        Self {
            key,
            questions,
            hash,
        }
    }

    fn hash_data(key: &str, questions: &[QuestionData]) -> String {
        let mut hasher = blake3::Hasher::new();

        hasher.update(key.as_bytes());
        hasher.update(
            questions
                .iter()
                .map(|question| question.hash.clone())
                .collect::<Vec<_>>()
                .join("")
                .as_bytes(),
        );

        hasher.finalize().to_string()
    }

    pub fn load_and_write_formatted(dir_entry: DirEntry) -> Result<Self> {
        let path = dir_entry.path();
        let mut data = Self::load(path.clone(), dir_entry)?;

        data.sort();
        data.clone().write(path)?;

        Ok(data)
    }

    pub fn load(path: PathBuf, dir_entry: DirEntry) -> Result<Self> {
        let raw_data = read_dir_entry_data(dir_entry)?;

        let key = path
            .file_stem()
            .and_then(|name| name.to_str())
            .expect("invalid file name")
            .to_owned();
        let questions = QuestionData::from_slice(&raw_data[..])?;

        Ok(Self::new(key, questions))
    }

    pub fn write(self, path: PathBuf) -> Result<()> {
        let raw_questions = self
            .questions
            .into_iter()
            .map(Into::into)
            .collect::<Vec<RawQuestionData>>();
        let raw_data = serde_json::to_string_pretty(&raw_questions)?;

        write_data(path, raw_data)
    }

    fn sort(&mut self) {
        self.questions
            .sort_by(|a, b| match a.evaluation.cmp(&b.evaluation) {
                Ordering::Equal => a.text.cmp(&b.text),
                ordering => ordering,
            });

        for question in self.questions.iter_mut() {
            OptionData::sort(&mut question.options);
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct QuestionData {
    pub id: Uuid,

    pub text: String,
    pub options: Vec<OptionData>,
    pub evaluation: String,

    pub hash: String,
}

impl QuestionData {
    fn new(id: Uuid, text: String, options: Vec<OptionData>, evaluation: String) -> Self {
        let hash = Self::hash_data(id, &text, &options[..], &evaluation);

        Self {
            id,
            text,
            options,
            evaluation,
            hash,
        }
    }

    fn hash_data(id: Uuid, text: &str, options: &[OptionData], evaluation: &str) -> String {
        let mut hasher = blake3::Hasher::new();

        hasher.update(id.as_bytes());
        hasher.update(text.as_bytes());
        hasher.update(
            options
                .iter()
                .map(|option| option.hash.clone())
                .collect::<Vec<_>>()
                .join("")
                .as_bytes(),
        );
        hasher.update(evaluation.as_bytes());

        hasher.finalize().to_string()
    }

    fn from_slice(raw_data: &[u8]) -> Result<Vec<Self>> {
        let raw_questions: Vec<RawQuestionData> = serde_json::from_slice(&raw_data)?;

        Ok(raw_questions.into_iter().map(Into::into).collect())
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

#[derive(Deserialize, Clone, Debug)]
pub struct OptionData {
    pub id: Uuid,

    pub text: String,
    pub correct: bool,

    pub hash: String,
}

impl OptionData {
    fn new(id: Uuid, text: String, correct: bool) -> Self {
        let hash = Self::hash_data(id, &text, correct);

        Self {
            id,
            text,
            correct,
            hash,
        }
    }

    fn hash_data(id: Uuid, text: &str, correct: bool) -> String {
        let mut hasher = blake3::Hasher::new();

        hasher.update(id.as_bytes());
        hasher.update(text.as_bytes());
        hasher.update(&[correct as u8]);

        hasher.finalize().to_string()
    }

    fn sort(options: &mut [Self]) {
        options.sort_by(|a, b| {
            if a.correct {
                Ordering::Less
            } else if b.correct {
                Ordering::Greater
            } else {
                a.text.cmp(&b.text)
            }
        })
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
