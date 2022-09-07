use std::path::PathBuf;

use anyhow::Result;
use secrecy::Secret;
use url::Url;

use medici_data_sync::data::CourseData;
use medici_data_sync::helpers::read_data_dir;

pub async fn sync(data_path: PathBuf, engine_url: Url, engine_key: Secret<String>) -> Result<()> {
    let entries = read_data_dir(data_path)?;

    let courses_data = entries
        .into_iter()
        .map(|dir_entry| CourseData::load_and_write_formatted(dir_entry?))
        .collect::<Result<Vec<_>>>()?;

    println!("{courses_data:?}");
    println!("{engine_url:?}");
    println!("{engine_key:?}");

    Ok(())
}
