use std::process::Command;

#[ path = "terminal/ui.rs" ] mod ui;

fn main() {
    Command::new("nmtui").arg("connect").status().expect("Failed to query NetworkManager.");
    ui::run();
}