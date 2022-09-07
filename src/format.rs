use std::path::PathBuf;

use anyhow::Result;

use medici_data_sync::data::CourseData;
use medici_data_sync::helpers::read_data_dir;

pub fn format(data_path: PathBuf) -> Result<()> {
    let entries = read_data_dir(data_path)?;

    for dir_entry in entries {
        CourseData::load_and_write_formatted(dir_entry?)?;
    }

    Ok(())
}
