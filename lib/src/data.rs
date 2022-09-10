use std::cmp::Ordering;
use std::fs::DirEntry;
use std::path::PathBuf;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::helpers::{read_dir_entry_data, write_data};
use crate::raw_data::{RawQuestionData, RawQuestionOptionData};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CourseData {
    pub key: String,

    #[serde(skip)]
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

        data.check()?;
        data.deduplicate();
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
                Ordering::Equal => match a.text.cmp(&b.text) {
                    Ordering::Equal => a.id.cmp(&b.id),
                    ordering => ordering,
                },
                ordering => ordering,
            });

        for question in self.questions.iter_mut() {
            question.sort_options();
        }
    }

    fn deduplicate(&mut self) {
        self.questions.dedup_by(|a, b| a.eq_data(b));

        for question in self.questions.iter_mut() {
            question.deduplicate_options();
        }
    }

    fn check(&self) -> Result<()> {
        for question in &self.questions {
            question.check()?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QuestionData {
    pub id: Uuid,

    pub course_key: Option<String>,
    pub text: String,
    #[serde(skip)]
    pub question_options: Vec<QuestionOptionData>,
    pub evaluation: String,

    pub hash: String,
}

impl QuestionData {
    fn new(
        id: Uuid,
        text: String,
        question_options: Vec<QuestionOptionData>,
        evaluation: String,
    ) -> Self {
        let hash = Self::hash_data(id, &text, &question_options[..], &evaluation);

        Self {
            id,
            course_key: None,
            text,
            question_options,
            evaluation,
            hash,
        }
    }

    fn hash_data(
        id: Uuid,
        text: &str,
        question_options: &[QuestionOptionData],
        evaluation: &str,
    ) -> String {
        let mut hasher = blake3::Hasher::new();

        hasher.update(id.as_bytes());
        hasher.update(text.as_bytes());
        hasher.update(
            question_options
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

    fn sort_options(&mut self) {
        self.question_options.sort_by(|a, b| {
            if a.correct {
                Ordering::Less
            } else if b.correct {
                Ordering::Greater
            } else {
                a.text.cmp(&b.text)
            }
        })
    }

    fn deduplicate_options(&mut self) {
        self.question_options.dedup_by(|a, b| a.eq_data(b));
    }

    fn eq_data(&self, other: &Self) -> bool {
        self.text == other.text
            && self.evaluation == other.evaluation
            && self.question_options.len() == other.question_options.len()
            && self
                .question_options
                .iter()
                .all(|a| other.question_options.iter().any(|b| a.eq_data(b)))
    }

    fn check(&self) -> Result<()> {
        if self.question_options.len() < 2 || self.question_options.len() > 5 {
            bail!(
                "Question {} has {} option(s)",
                self.id,
                self.question_options.len()
            );
        }

        let correct_count = self
            .question_options
            .iter()
            .filter(|option| option.correct)
            .count();

        if correct_count != 1 {
            bail!("Question {} has {correct_count} correct options", self.id)
        }

        Ok(())
    }
}

impl From<RawQuestionData> for QuestionData {
    fn from(raw: RawQuestionData) -> Self {
        let options = raw.question_options.into_iter().map(Into::into).collect();

        Self::new(
            raw.id.unwrap_or_else(|| Uuid::new_v4()),
            raw.text,
            options,
            raw.evaluation,
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QuestionOptionData {
    pub id: Uuid,

    pub question_id: Option<Uuid>,
    pub text: String,
    pub correct: bool,
    pub explanation: Option<String>,

    pub hash: String,
}

impl QuestionOptionData {
    fn new(id: Uuid, text: String, correct: bool, explanation: Option<String>) -> Self {
        let hash = Self::hash_data(id, &text, correct, explanation.as_deref());

        Self {
            id,
            question_id: None,
            text,
            correct,
            explanation,
            hash,
        }
    }

    fn hash_data(id: Uuid, text: &str, correct: bool, explanation: Option<&str>) -> String {
        let mut hasher = blake3::Hasher::new();

        hasher.update(id.as_bytes());
        hasher.update(text.as_bytes());
        hasher.update(&[correct as u8]);

        if let Some(explanation) = explanation {
            hasher.update(explanation.as_bytes());
        }

        hasher.finalize().to_string()
    }

    fn eq_data(&self, other: &Self) -> bool {
        self.text == other.text
            && self.correct == other.correct
            && self.explanation == other.explanation
    }
}

impl From<RawQuestionOptionData> for QuestionOptionData {
    fn from(raw: RawQuestionOptionData) -> Self {
        Self::new(
            raw.id.unwrap_or_else(|| Uuid::new_v4()),
            raw.text,
            raw.correct.unwrap_or(false),
            raw.explanation,
        )
    }
}
