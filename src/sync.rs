use std::path::PathBuf;

use anyhow::Result;

use medici_data_sync::data::CourseData;
use medici_data_sync::helpers::read_data_dir;

pub async fn sync(data_path: PathBuf) -> Result<()> {
    let entries = read_data_dir(data_path)?;

    let courses_data = entries
        .into_iter()
        .map(|dir_entry| CourseData::load_and_write_formatted(dir_entry?))
        .collect::<Result<Vec<_>>>()?;

    println!("{courses_data:?}");

    Ok(())
}
