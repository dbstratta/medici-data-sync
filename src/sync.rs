use std::{collections::HashMap, path::PathBuf};

use anyhow::{bail, Result};
use secrecy::{ExposeSecret, Secret};
use url::Url;

use medici_data_sync::{read_data_dir, CourseData, CourseMetadata, SyncData};

pub async fn sync(data_path: PathBuf, engine_url: Url, engine_key: Secret<String>) -> Result<()> {
    let engine_client = engine_client(engine_key)?;
    let courses_metadata = courses_metadata(&engine_client, engine_url.clone()).await?;

    let entries = read_data_dir(data_path)?;

    let courses_data = entries
        .into_iter()
        .map(|dir_entry| CourseData::load_and_write_formatted(dir_entry?))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(|data| {
            if let Some(metadata) = courses_metadata.get(&data.key) {
                if metadata.hash == data.hash {
                    return false;
                }
            }

            true
        })
        .collect::<Vec<_>>();

    sync_courses(
        &engine_client,
        engine_url.clone(),
        SyncData {
            to_update: courses_data,
            to_delete: vec![],
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

async fn sync_courses(client: &reqwest::Client, engine_url: Url, data: SyncData) -> Result<()> {
    let url = engine_url.join("sync-courses")?;
    let response = client.post(url).json(&data).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        bail!("Error {}", response.status())
    }
}
