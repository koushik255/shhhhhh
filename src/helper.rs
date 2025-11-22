use std::io;
use std::{
    fs::{DirEntry, read_dir},
    path::PathBuf,
};

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct BetterFile {
    pub file_path: PathBuf,
    pub file_extention: String,
    pub file_name: String,
}

//takes reference to DirEntry and returns the filename without path and exteion and the extension
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
        //println!("No File extension found");
    }

    let mut metoo = "none";
    if let Some((_, want)) = s.rsplit_once('/') {
        metoo = want;
    } else {
        //println!("No able to get the filename without path");
    }

    (fuckyou.to_owned(), metoo.to_owned())
}

pub fn everything(path: String) {
    let mut files: Vec<BetterFile> = Vec::new();

    let path1 = path.clone();
    let mut pathmine = "/home/koushikk/Downloads/SHOWS/";
    if pathmine.to_string() != path {
        pathmine = path1.as_str();
    }
    let efwa: Result<Vec<PathBuf>, io::Error> = read_dir(pathmine)
        .expect("Error reading Directory")
        .map(|file| {
            file.map(|f| {
                if f.path().is_dir() {
                    handle_dir(&f);
                    //dir
                } else {
                    let (ext, fname) = extenshik(&f);
                    let brutha = BetterFile {
                        file_path: f.path(),
                        file_extention: ext,
                        file_name: fname,
                    };

                    files.push(brutha);
                };
                f.path()
            })
        })
        .collect();

    let mut efwa = efwa.expect("Error collecting files");

    efwa.sort();
    files.sort();
    files.iter().for_each(|file| {
        println!("File{}", file.file_path.display());
    });
}

pub fn handle_dir(file_path: &DirEntry) {
    let dir_to_read = file_path.path().to_path_buf();
    let efwa: Result<Vec<PathBuf>, io::Error> = read_dir(dir_to_read)
        .expect("Error reading Directory")
        .map(|file| file.map(|f| f.path()))
        .collect();
    // il put rayon in later
    // remeber this is a crate no its not but im going to remake gtk4shell with better code
    // oh yeah i need to use rc and rcell to add them to the same vec

    let efwa = efwa.expect("error dir");
    efwa.iter().for_each(|f| {
        if f.is_dir() {
            println!("dir-dir");
            everything(f.to_str().unwrap().to_string());
        } else {
            println!("FilefromDir {}", f.display());
        }
    });
}

pub fn greet() {
    println!("from helper");
}
