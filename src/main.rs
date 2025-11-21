use std::{fs::read_dir, io, path::PathBuf};

use crate::helper::{BetterFile, extenshik, greet};
mod helper;

fn main() {
    println!("Hello, world!");
    greet();

    let mut files: Vec<BetterFile> = Vec::new();

    let path = "/home/koushikk/Downloads/SHOWS/owairimonoseasontwo/";
    let efwa: Result<Vec<PathBuf>, io::Error> = read_dir(path)
        .expect("Error reading Directory")
        .map(|file| {
            file.map(|f| {
                if f.path().is_dir() {
                    // this is where i would send off the threads for async recursion
                    println!("this a dir baby brol");
                }
                let (ext, fname) = extenshik(&f);
                let brutha = BetterFile {
                    file_path: f.path(),
                    file_extention: ext,
                    file_name: fname,
                };
                files.push(brutha);

                f.path()
                //bonbaba
            })
        })
        .collect();

    let mut efwa = efwa.expect("Error collecting files");

    efwa.sort();
    files.iter().for_each(|file| {
        println!(
            "File {}, Ext {}, FileName {}",
            file.file_path.display(),
            file.file_extention,
            file.file_name
        )
    });
}
