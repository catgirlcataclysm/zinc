use std::process::Command;

mod zinc;
mod install;
mod hardware;

pub const BOARDS: [&'static str; 10] = ["bob", "coachz", "hana", "homestar", "kevin", "kodama", "krane", "lazor", "minnie", "speedy"];
pub const BASEBOARDS: [&'static str; 5] = ["gru", "kukui", "oak", "trogdor", "veyron"];
pub const ARCH: Option<&'static str> = option_env!("CARGO_CFG_TARGET_ARCH");

fn main() {
    Command::new("nmtui").arg("connect").status().expect("Failed to query NetworkManager.");
    zinc::run();
}