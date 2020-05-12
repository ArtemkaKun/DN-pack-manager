use std::{ffi::OsStr, fs, path::Path};

pub fn search_pack(path_to_packs: String) -> Vec<String> {
    search_in_dir(path_to_packs)
}

fn search_in_dir(path: String) -> Vec<String> {
    let dir_path = Path::new(&path);

    let mut packs = Vec::<String>::new();

    for one_file in fs::read_dir(dir_path).expect("Unable to list or connect") {
        let one_file = one_file.expect("Unable to get file");

        if one_file.path().extension() == Some(OsStr::new("pak")) {
            packs.push(one_file.path().display().to_string());
        }
    }

    packs
}
