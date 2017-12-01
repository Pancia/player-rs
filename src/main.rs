#[macro_use]
extern crate clap;
extern crate itertools;
extern crate rand;

use clap::{App, Arg};
use rand::Rng;

use std::process::Command;
use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;

fn shuffle<T>(coll: &mut Vec<T>) {
    let mut rng = rand::thread_rng();
    rng.shuffle(coll);
}

#[derive(Debug)]
struct TetrisShuffle<T> {
    coll: Vec<T>,
    len: usize,
    idx: usize,
}

impl<T> TetrisShuffle<T> {
    fn new(mut coll: Vec<T>) -> TetrisShuffle<T> {
        let len = coll.len();
        shuffle(&mut coll);
        TetrisShuffle {
            coll,
            len: len,
            idx: 0,
        }
    }
}

impl<T: Clone + Eq> Iterator for TetrisShuffle<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let next_item = self.coll[self.idx].clone();
        if self.len == 1 {
            return Some(next_item);
        } else if self.idx == self.len - 1 {
            self.idx = 0;
            shuffle(&mut self.coll);
            while self.coll[0] == next_item {
                shuffle(&mut self.coll);
            }
        } else {
            self.idx += 1;
        }
        Some(next_item)
    }
}

fn tetris_shuffle<T>(coll: Vec<T>) -> TetrisShuffle<T> {
    TetrisShuffle::new(coll)
}

#[cfg(test)]
mod tests {
    use tetris_shuffle;
    use itertools::Itertools;

    #[test]
    fn tetris_shuffle_works() {
        let to_shuffle = vec![1, 2, 3, 4, 5];
        let shuffled: Vec<usize> = tetris_shuffle(to_shuffle)
            .chunks(5)
            .into_iter()
            .take(3)
            .map(|xs| xs.sum())
            .collect();
        assert_eq!(
            shuffled,
            [15, 15, 15],
            "grouped in groups of the original collection length"
        )
    }
}

fn expand_dirs(path: &Path) -> Vec<PathBuf> {
    if fs::metadata(path).unwrap().is_dir() {
        //clone or use PathBuf
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

    for file in tetris_shuffle(music_files) {
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
