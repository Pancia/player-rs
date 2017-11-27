extern crate rand;
#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::process::Command;

fn main() {
    //let mut rng = rand::thread_rng();

    let matches = App::new("player-rs")
        .version(crate_version!())
        .arg(Arg::with_name("volume")
             .short("v")
             .value_name("VOLUME"))
        .arg(Arg::with_name("INPUT")
                .required(true)
                .index(1))
        .get_matches();

    let music_file = matches.value_of("INPUT").unwrap();
    let volume_arg = value_t!(matches, "volume", u32).unwrap_or(5);
    let volume_str = &(volume_arg as f32 / 100.0).to_string();

    let ecode = Command::new("afplay")
        .arg(music_file)
        .args(&["-v", volume_str])
        .spawn()
        .expect("failed to execute afplay")
        .wait()
        .expect("failed to wait for afplay");

    assert!(ecode.success());
}
