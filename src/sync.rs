use std::{collections::HashSet, path::PathBuf};

use anyhow::{bail, Result};
use secrecy::{ExposeSecret, Secret};
use url::Url;

use medici_data_sync::{
    load_courses_data_and_write_formatted, CourseEvaluationData, SyncData, SyncMetadata,
};

pub async fn sync(data_path: PathBuf, engine_url: Url, engine_key: Secret<String>) -> Result<()> {
    let engine_client = engine_client(engine_key)?;
    let mut sync_metadata = sync_metadata(&engine_client, engine_url.clone()).await?;

    let mut courses_to_sync = vec![];
    let mut questions_to_sync = vec![];
    let mut question_options_to_sync = vec![];
    let mut course_evaluations_to_sync = HashSet::<CourseEvaluationData>::new();

    let mut courses_data = load_courses_data_and_write_formatted(data_path)?;

    for mut course_data in courses_data.drain(..) {
        let skip_course = match sync_metadata.courses_metadata.remove(&course_data.key) {
            Some(course_hash) if course_hash == course_data.hash => true,
            _ => false,
        };

        for mut question_data in course_data.questions.drain(..) {
            question_data.set_course_key(course_data.key.clone());
            sync_metadata
                .course_evaluations
                .remove(&question_data.evaluation);

            let skip_question = match sync_metadata.questions_metadata.remove(&question_data.id) {
                Some(question_hash) if question_hash == question_data.hash => true,
                _ if !skip_course => false,
                _ => true,
            };

            for mut question_option_data in question_data.question_options.drain(..) {
                question_option_data.set_question_id(question_data.id);

                match sync_metadata
                    .question_options_metadata
                    .remove(&question_option_data.id)
                {
                    Some(question_option_hash)
                        if question_option_hash == question_option_data.hash => {}
                    _ => question_options_to_sync.push(question_option_data),
                }
            }

            if !skip_question {
                course_evaluations_to_sync.insert(CourseEvaluationData {
                    course_key: course_data.key.clone(),
                    key: question_data.evaluation.clone(),
                });
                questions_to_sync.push(question_data);
            }
        }

        if !skip_course {
            courses_to_sync.push(course_data.clone());
        }
    }

    let courses_to_delete = sync_metadata.courses_metadata.keys().cloned().collect();
    let questions_to_delete = sync_metadata.questions_metadata.keys().cloned().collect();
    let question_options_to_delete = sync_metadata
        .question_options_metadata
        .keys()
        .cloned()
        .collect();

    let used_course_evaluation_keys = course_evaluations_to_sync
        .iter()
        .map(|data| data.key.clone())
        .collect();
    let course_evaluations_to_delete = sync_metadata
        .course_evaluations
        .difference(&used_course_evaluation_keys)
        .cloned()
        .collect();

    sync_data(
        &engine_client,
        engine_url.clone(),
        SyncData {
            courses_to_sync,
            courses_to_delete,

            questions_to_sync,
            questions_to_delete,

            question_options_to_sync,
            question_options_to_delete,

            course_evaluations_to_sync,
            course_evaluations_to_delete,
        },
    )
    .await?;

    Ok(())
}

fn engine_client(engine_key: Secret<String>) -> Result<reqwest::Client> {
    let client = reqwest::Client::builder()
        .default_headers(
            [(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", engine_key.expose_secret()).parse()?,
            )]
            .into_iter()
            .collect(),
        )
        .build()?;

    Ok(client)
}

async fn sync_metadata(client: &reqwest::Client, engine_url: Url) -> Result<SyncMetadata> {
    let url = engine_url.join("sync-metadata")?;

    Ok(client.get(url).send().await?.json().await?)
}

async fn sync_data(client: &reqwest::Client, engine_url: Url, data: SyncData) -> Result<()> {
    let url = engine_url.join("sync-data")?;
    let response = client.post(url).json(&data).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        bail!("Error {}", response.status())
    }
}
