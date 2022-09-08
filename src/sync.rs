use std::path::PathBuf;

use anyhow::Result;
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
    let courses_metadata = courses_metadata(engine_client, &engine_url).await?;

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

async fn courses_metadata(client: reqwest::Client, engine_url: &Url) -> Result<CourseMetadata> {
    let mut url = engine_url.clone();
    url.set_path("");

    Ok(client.get(url).send().await?.json().await?)
}
