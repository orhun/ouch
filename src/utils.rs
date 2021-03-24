use std::{fs, path::{Path, PathBuf}};

use colored::Colorize;
use crate::{error::{Error, OuchResult}, extension::CompressionFormat, file::File};

pub (crate) fn ensure_exists<'a, P>(path: P) -> OuchResult<()>
where
    P: AsRef<Path> + 'a   {
        let exists = path.as_ref().exists();
        if !exists {
            eprintln!("{}: could not find file {:?}", "error".red(), path.as_ref());
        }
        Ok(())
    }

pub (crate) fn check_for_multiple_files(files: &Vec<PathBuf>, format: &CompressionFormat) -> OuchResult<()> {

    if files.len() != 1 {
        eprintln!("{}: cannot compress multiple files directly to {:#?}.\n       Try using an intermediate archival method such as Tar.\n       Example: filename.tar{}", "error".red(), format, format);
        return Err(Error::InvalidInput);
    }

    Ok(())
}

pub (crate) fn create_path_if_non_existent(path: &Path) -> OuchResult<()> {
    if !path.exists() {
        println!(
            "{}: attempting to create folder {:?}.",
            "info".yellow(),
            &path
        );
        std::fs::create_dir_all(path)?;
        println!(
            "{}: directory {:#?} created.",
            "info".yellow(),
            fs::canonicalize(&path)?
        );
    }
    Ok(())
}

pub (crate) fn get_destination_path(dest: &Option<File>) -> &Path {
    match dest {
        Some(output) => {
            // Must be None according to the way command-line arg. parsing in Ouch works
            assert_eq!(output.extension, None);

            Path::new(&output.path)
        }
        None => Path::new("."),
    }
}