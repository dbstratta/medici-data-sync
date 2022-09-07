use std::path::PathBuf;

use anyhow::Result;

use crate::data::CourseData;
use crate::helpers::read_data_dir;

pub fn sync(data_path: PathBuf) -> Result<()> {
    let entries = read_data_dir(data_path)?;

    for dir_entry in entries {
        let course_data = CourseData::load_and_write_formatted(dir_entry?)?;

        println!("{:?}", course_data.key);
        println!("{:?}", course_data.hash);
        println!("{:?}", course_data);
    }

    Ok(())
}
