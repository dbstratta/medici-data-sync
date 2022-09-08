use std::{collections::HashMap, path::PathBuf};

use anyhow::{bail, Result};
use secrecy::{ExposeSecret, Secret};
use url::Url;

use medici_data_sync::{read_data_dir, CourseData, CourseMetadata};

pub async fn sync(data_path: PathBuf, engine_url: Url, engine_key: Secret<String>) -> Result<()> {
    let entries = read_data_dir(data_path)?;

    let courses_data = entries
        .into_iter()
        .map(|dir_entry| CourseData::load_and_write_formatted(dir_entry?))
        .collect::<Result<Vec<_>>>()?;

    let engine_client = engine_client(engine_key)?;
    let courses_metadata = courses_metadata(&engine_client, engine_url.clone()).await?;

    for course_data in courses_data {
        sync_course_data(
            &engine_client,
            engine_url.clone(),
            course_data.clone(),
            courses_metadata.get(&course_data.key),
        )
        .await?;
    }

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

async fn courses_metadata(
    client: &reqwest::Client,
    engine_url: Url,
) -> Result<HashMap<String, CourseMetadata>> {
    let url = engine_url.join("courses-metadata")?;

    let courses_metadata = client
        .get(url)
        .send()
        .await?
        .json::<Vec<CourseMetadata>>()
        .await?
        .into_iter()
        .map(|metadata| (metadata.key.clone(), metadata))
        .collect();

    Ok(courses_metadata)
}

async fn sync_course_data(
    client: &reqwest::Client,
    engine_url: Url,
    course_data: CourseData,
    metadata: Option<&CourseMetadata>,
) -> Result<()> {
    if let Some(metadata) = metadata {
        if metadata.hash == course_data.hash {
            return Ok(());
        }
    }

    let url = engine_url.join("update-course-data")?;
    let response = client.post(url).json(&course_data).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        bail!("Error {}", response.status())
    }
}
