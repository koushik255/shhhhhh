use std::{fs::DirEntry, path::PathBuf};

pub struct BetterFile {
    pub file_path: PathBuf,
    pub file_extention: String,
    pub file_name: String,
}

pub fn extenshik(file_path: &DirEntry) -> (String, String) {
    let s = file_path
        .to_owned()
        .path()
        .as_mut_os_string()
        .clone()
        .into_string()
        .unwrap();
    let mut fuckyou = "supdeude";
    if let Some((_, s3)) = s.rsplit_once('.') {
        fuckyou = s3;
    } else {
        println!("No File extension found");
    }

    let mut metoo = "none";
    if let Some((_, want)) = s.rsplit_once('/') {
        metoo = want;
    } else {
        println!("No able to get the filename without path");
    }

    (fuckyou.to_owned(), metoo.to_owned())
}

pub fn greet() {
    println!("from helper");
}
