use std::io;
use std::ops::Deref;
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
// lf = look for
pub fn everything(path: String, lf: String) -> Vec<BetterFile> {
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
                    // dude its wraps
                    // println!("nothgin");
                } else {
                    if f.path().is_dir() {
                        let dir_files = handle_dir(&f, lf.clone());
                        println!("STUCK IN NOtPRIVATE {:?}", f);
                        for file in dir_files {
                            files.push(file);
                        }
                        //dir
                    } else {
                        let (ext, fname) = extenshik(&f);
                        if ext == lf {
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
    // i cna try regex for matcing the "./"

    // let mut efwa = read_dir(pathmine).into_par_iter().filter_map(|f| {});
    // let me just try regex first for the private directory checking
    // release is taking less than 1 secound but i need to be able to skip over certain directorys
    // but what would that even look like and how? also i need to check if the "sorting" like into
    // betterfile is the thing which is sloinwg it down
    // also maybe hashmap would be faster since im just inserting into the hash whereas with the
    // vecs im creating a vec then, or right now my match is basically just running thorugh
    // EVERYthing again right? i should first get the W files then match those "ext" ws with the
    // new search
    //
    // self hosted music app seems fucking goated i should make something like that
    // -- andriod/ios developopment + server again
    // yeah i like that ALOT
    // i should get video working first, i mean i kinda did right?
    // look into that while chelsea game
    // also i always want to keep the programming on my laptop so how i do that, but also keep it
    // on the computer aswell
    //

    //efwa.sort();

    files.sort();
    files.iter().for_each(|file| {
        println!(
            "File{} Ext .{}",
            file.file_path.display(),
            file.file_extention
        );
        // should make everything lower
    });

    let filecheck = &files;
    let these_ones = filecheck.deref().iter().par_bridge().filter_map(|f| {
        if f.file_name.to_lowercase().contains("mono") {
            //println!("we got a franx diddy");
            Some(f)
            //None
        } else {
            //println!("none");
            //
            None
        }
    });
    let check_complete: Vec<&BetterFile> = these_ones.to_owned().collect();
    check_complete
        .iter()
        .for_each(|f| println!("MATCH : {}", f.file_path.display()));

    files
}

pub fn handle_dir(file_path: &DirEntry, lf: String) -> Vec<BetterFile> {
    let dir_to_read = file_path.path().to_path_buf();
    let dir_string = dir_to_read.to_string_lossy().to_string();

    let mut files: Vec<BetterFile> = Vec::new();

    if dir_string.contains(".var") {
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
                    handle_dir(f, lf.clone()) // recus
                } else {
                    let (ext, fname) = extenshik(f);
                    if ext == lf.as_str() {
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

pub fn greet() {
    println!("from helper");
}
