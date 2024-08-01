use std::{fs::File, process::Command};

use install::debug_output;
use simplelog::{Config, WriteLogger};

mod hardware;
mod install;
mod zinc;

pub const BOARDS: [&str; 10] = [
    "bob", "coachz", "hana", "homestar", "kevin", "kodama", "krane", "lazor", "minnie", "speedy",
];
pub const BASEBOARDS: [&str; 5] = ["gru", "kukui", "oak", "trogdor", "veyron"];
fn main() {
    WriteLogger::init(
        log::LevelFilter::Debug,
        Config::default(),
        File::create("zinc.log").expect("Failed to create zinc.log"),
    )
    .expect("Failed to initialise logger.");

    Command::new("nmtui")
        .arg("connect")
        .status()
        .expect("Failed to query NetworkManager.");
    let output = Command::new("umount")
        .arg("/mnt")
        .output()
        .expect("Failed to unmount /mnt");
    debug_output(output);
    zinc::run();
}
