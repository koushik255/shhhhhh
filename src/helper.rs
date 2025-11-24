use std::io;
use std::{
    fs::{DirEntry, read_dir},
    path::PathBuf,
};

use rayon::prelude::*;

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

// i could just have both functions return a vec and do it like that, or i have them use rc rc
// might be worse il use rc later
pub fn everything(path: String) -> Vec<BetterFile> {
    let mut files: Vec<BetterFile> = Vec::new();

    let path1 = path.clone();
    let mut pathmine = "/home/";
    if pathmine.to_string() != path {
        pathmine = path1.as_str();
    }
    let efwa: Result<Vec<PathBuf>, io::Error> = read_dir(pathmine)
        .expect("Error reading Directory")
        .map(|file| {
            file.map(|f| {
                //println!("stuck on {:?}", f.path().file_stem()); // stuck on /.var so need to ignore /.{private} dirs
                let current_dir = f.path().file_stem().unwrap().to_string_lossy().into_owned();
                if current_dir.contains('.') {
                    // try filter instead of what i have right now i would do filter.map does not contain "."
                } else {
                    if f.path().is_dir() {
                        let dir_files = handle_dir(&f);
                        println!("STUCK IN NONE PRIVATE {:?}", f);
                        for file in dir_files {
                            files.push(file);
                        }
                        //dir
                    } else {
                        let (ext, fname) = extenshik(&f);
                        if ext == "mkv".to_string() {
                            let brutha = BetterFile {
                                file_path: f.path(),
                                file_extention: ext,
                                file_name: fname,
                            };

                            files.push(brutha);
                        }
                    };
                }
                f.path()
            })
        })
        .collect();

    // let mut efwa = efwa.expect("Error collecting files");

    //efwa.sort();

    files.sort();
    files.iter().for_each(|file| {
        println!(
            "File{} Ext .{}",
            file.file_path.display(),
            file.file_extention
        );
    });
    files
}

pub fn handle_dir(file_path: &DirEntry) -> Vec<BetterFile> {
    let dir_to_read = file_path.path().to_path_buf();
    let dir_string = dir_to_read.to_string_lossy().to_string();

    let mut files: Vec<BetterFile> = Vec::new();

    if dir_string.contains('.') {
        //println!("Private skip this ");
    } else {
        //println!("DIR FROM HANDLE DIR {}", dir_to_read.display());
        let efwa: Result<Vec<DirEntry>, io::Error> = read_dir(dir_to_read)
            .expect("Error reading Directory")
            .collect();

        let efwa = efwa.expect("error dir");

        files = efwa
            .par_iter()
            .flat_map(|f| {
                if f.path().is_dir() {
                    //println!("dir {}", f.path().display());
                    handle_dir(f) // recus
                } else {
                    let (ext, fname) = extenshik(f);
                    if ext == "mkv" {
                        vec![BetterFile {
                            file_path: f.path(),
                            file_extention: ext,
                            file_name: fname,
                        }]
                    } else {
                        vec![]
                    }
                }
            })
            .collect();
    }
    files
}
/// next thing feature should be if if the dir has like alot of files we skip it and just show the
/// dir, would be a "too big" check  i could run the each "too big " in a task spwaned by smol
/// so how would i check if a dir should not be looped over?
pub fn greet() {
    println!("from helper");
}
