extern crate rand;
#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::process::Command;
use std::io;

fn main() {
    //let mut rng = rand::thread_rng();

    let matches = App::new("player-rs")
        .version(crate_version!())
        .arg(Arg::with_name("volume")
             .short("v")
             .value_name("VOLUME"))
        .arg(Arg::with_name("INPUT")
                .required(true)
                .multiple(true))
        .get_matches();

    let music_files: Vec<&str> = matches.values_of("INPUT").unwrap().collect();
    let volume_arg = value_t!(matches, "volume", u32).unwrap_or(5);
    let volume_str = &(volume_arg as f32 / 100.0).to_string();

    //TODO: make music_files an infinite iterator over its shuffle
    for file in music_files {
        println!("Playing {}", file);
        let mut child = Command::new("afplay")
            .arg(file)
            .args(&["-v", volume_str])
            .spawn()
            .expect("failed to execute afplay");

        while {
            let exit = child.try_wait();
            if exit.is_ok() { exit.unwrap().is_none() } else { true }
        } {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("failed to get some input");
            if input == "\n" {
                child.kill().expect("was no afplay?");
                child.wait().unwrap();
            }
        }
    }

    println!("K THX BAI!");
}
