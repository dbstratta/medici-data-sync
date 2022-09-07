use std::fs::{self, DirEntry, ReadDir};
use std::path::PathBuf;

use anyhow::{bail, Result};

pub fn read_data_dir(data_path: PathBuf) -> Result<ReadDir> {
    let data_path = fs::canonicalize(data_path)?;
    let entries = fs::read_dir(data_path)?;

    Ok(entries)
}

pub fn read_dir_entry_data(dir_entry: DirEntry) -> Result<Vec<u8>> {
    if dir_entry.file_type()?.is_dir() {
        bail!("");
    };

    Ok(fs::read(dir_entry.path())?)
}

pub fn write_data(path: PathBuf, data: String) -> Result<()> {
    fs::write(path, format!("{data}\n"))?;

    Ok(())
}
