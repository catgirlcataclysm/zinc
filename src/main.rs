use std::process::Command;

use simple_logger::SimpleLogger;

mod hardware;
mod install;
mod zinc;

pub const BOARDS: [&str; 10] = [
    "bob", "coachz", "hana", "homestar", "kevin", "kodama", "krane", "lazor", "minnie", "speedy",
];
pub const BASEBOARDS: [&str; 5] = ["gru", "kukui", "oak", "trogdor", "veyron"];
fn main() {
    let _ = SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .init();

    Command::new("nmtui")
        .arg("connect")
        .status()
        .expect("Failed to query NetworkManager.");
    zinc::run();
}
