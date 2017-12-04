#[macro_use]
extern crate clap;
#[cfg(test)]
extern crate itertools;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate rand;

mod shuffle;

use shuffle::Shuffle;

use clap::{App, Arg};

use std::process::Command;
use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;

fn expand_dirs(path: &Path) -> Vec<PathBuf> {
    if fs::metadata(path).unwrap().is_dir() {
        fs::read_dir(path)
            .unwrap()
            .map(|r| r.unwrap().path())
            .collect()
    } else {
        vec![path.to_path_buf()]
    }
}

fn main() {
    let matches = App::new("player-rs")
        .version(crate_version!())
        .arg(Arg::with_name("volume").short("v").value_name("VOLUME"))
        .arg(Arg::with_name("INPUT").required(true).multiple(true))
        .get_matches();

    let input_paths: Vec<&str> = matches.values_of("INPUT").unwrap().collect();
    let music_files: Vec<PathBuf> = input_paths
        .iter()
        .map(|s| Path::new(s))
        .flat_map(expand_dirs)
        .collect();

    let volume_arg = value_t!(matches, "volume", u32).unwrap_or(5);
    let volume_str = &(volume_arg as f32 / 100.0).to_string();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("stdin");
        match input {
            _ => tx.send("kill").unwrap(),
        }
    });

    for file in Shuffle::new(music_files) {
        println!("Playing {}", file.display());

        let mut child = Command::new("afplay")
            .arg(file)
            .args(&["-v", volume_str])
            .spawn()
            .expect("failed to execute afplay");

        while child.try_wait().ok().map(|x| !x.is_some()).unwrap_or(true) {
            let _ = rx.try_recv().map(|_| {
                let _ = child.kill();
                child.wait().expect("failed to kill afplay");
            });
        }
    }

    println!("K THX BAI!");
}
