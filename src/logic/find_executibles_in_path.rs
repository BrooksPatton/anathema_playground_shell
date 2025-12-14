use std::{
    env,
    fs::{self, DirEntry},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

pub fn find_executibles_in_path() -> Vec<DirEntry> {
    get_path()
        .into_iter()
        .filter_map(|path| {
            let entries = fs::read_dir(path).ok()?;
            let executables = entries
                .filter_map(|file| {
                    let file = file.ok()?;

                    if file_is_executable(&file) {
                        Some(file)
                    } else {
                        None
                    }
                })
                .collect::<Vec<DirEntry>>();

            Some(executables)
        })
        .flatten()
        .collect()
}

fn get_path() -> Vec<PathBuf> {
    let path_environment = env::var("PATH").unwrap_or_default();
    let paths = path_environment.split(":");

    paths
        .map(Path::new)
        .map(|path| path.to_path_buf())
        .filter(|path| path.is_dir())
        .collect()
}

fn file_is_executable(dir_entry: &DirEntry) -> bool {
    let Ok(metadata) = dir_entry.metadata() else {
        return false;
    };
    let mode = metadata.mode();
    let user_can_execute = mode & 0o100 != 0;
    let group_can_execute = mode & 0o010 != 0;
    let other_can_execute = mode & 0o001 != 0;

    user_can_execute || group_can_execute || other_can_execute
}

pub fn find_first_executable_in_path_by_name(filename: &str) -> Option<DirEntry> {
    let executable_files = find_executibles_in_path();

    executable_files.into_iter().find(|executable_file| {
        executable_file.file_name().to_str().unwrap_or_default() == filename
    })
}
