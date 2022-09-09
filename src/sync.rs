use std::path::PathBuf;

use anyhow::{bail, Result};
use secrecy::{ExposeSecret, Secret};
use url::Url;

use medici_data_sync::{read_data_dir, CourseData, SyncData, SyncMetadata};

pub async fn sync(data_path: PathBuf, engine_url: Url, engine_key: Secret<String>) -> Result<()> {
    let engine_client = engine_client(engine_key)?;
    let mut sync_metadata = sync_metadata(&engine_client, engine_url.clone()).await?;

    let mut courses_to_sync = vec![];
    let mut questions_to_sync = vec![];
    let mut options_to_sync = vec![];

    let mut courses_data = read_data_dir(data_path)?
        .into_iter()
        .map(|dir_entry| CourseData::load_and_write_formatted(dir_entry?))
        .collect::<Result<Vec<_>>>()?;

    for mut course_data in courses_data.drain(..) {
        if let Some(course_hash) = sync_metadata.courses_metadata.remove(&course_data.key) {
            if course_hash == course_data.hash {
                continue;
            }
        }

        for mut question_data in course_data.questions.drain(..) {
            if let Some(question_hash) = sync_metadata.questions_metadata.remove(&question_data.id)
            {
                if question_hash == question_data.hash {
                    continue;
                }
            }

            for option_data in question_data.options.drain(..) {
                if let Some(option_hash) = sync_metadata.options_metadata.remove(&option_data.id) {
                    if option_hash == option_data.hash {
                        continue;
                    }
                }

                options_to_sync.push(option_data);
            }

            questions_to_sync.push(question_data);
        }

        courses_to_sync.push(course_data);
    }

    let courses_to_delete = sync_metadata.courses_metadata.keys().cloned().collect();
    let questions_to_delete = sync_metadata.questions_metadata.keys().cloned().collect();
    let options_to_delete = sync_metadata.options_metadata.keys().cloned().collect();

    sync_courses(
        &engine_client,
        engine_url.clone(),
        SyncData {
            courses_to_sync,
            courses_to_delete,

            questions_to_sync,
            questions_to_delete,

            options_to_sync,
            options_to_delete,
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

async fn sync_courses(client: &reqwest::Client, engine_url: Url, data: SyncData) -> Result<()> {
    let url = engine_url.join("sync-courses")?;
    let response = client.post(url).json(&data).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        bail!("Error {}", response.status())
    }
}
