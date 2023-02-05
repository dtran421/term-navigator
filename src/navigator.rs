use std::{
    fs::{self, DirEntry},
    io::Error,
    path::PathBuf,
};

pub fn valid_path(entry: &DirEntry, all: bool) -> bool {
    let path = entry.path();

    if path.is_dir() {
        let pathname = path
            .file_name()
            .expect("file name exists")
            .to_str()
            .expect("file name converts to str");
        if all || !pathname.starts_with(".") {
            return true;
        }
    }

    return false;
}

pub fn get_path_options(path: &PathBuf, no_index: bool, all: bool) -> Result<Vec<String>, Error> {
    let paths = fs::read_dir(path.to_owned());

    let path_options = paths?
        .into_iter()
        .filter_map(|res| res.ok())
        .filter(|entry| valid_path(entry, all))
        .map(|entry| {
            entry
                .path()
                .file_stem()
                .expect("get file stem from path")
                .to_str()
                .expect("convert path to string")
                .to_owned()
        })
        .collect::<Vec<String>>();
    let default_options = [".".into(), "..".into()].to_vec();

    let mut options = [default_options, path_options].concat();
    if !no_index {
        for i in 0..options.len() {
            let mut num_option = format!("[{}] ", i);
            num_option.push_str(&options[i]);
            options[i] = num_option;
        }
    }

    return Ok(options);
}
