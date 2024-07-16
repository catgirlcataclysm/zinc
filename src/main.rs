use std::process::Command;

mod ui;

fn main() {
    Command::new("nmtui").arg("connect").status().expect("Failed to query NetworkManager.");
    ui::run();
}