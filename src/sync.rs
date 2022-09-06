use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Result};

use crate::raw_data::RawQuestionData;

pub fn sync(data_path: PathBuf) -> Result<()> {
    let data_path = fs::canonicalize(data_path)?;
    let entries = fs::read_dir(data_path)?;

    for dir_entry in entries {
        let entry = dir_entry?;

        if entry.file_type()?.is_dir() {
            bail!("");
        };

        let raw_data = fs::read(entry.path())?;

        let questions: Vec<RawQuestionData> = serde_json::from_slice(&raw_data)?;

        println!("{:?}", questions);
    }

    Ok(())
}
