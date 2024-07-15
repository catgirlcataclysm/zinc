use cursive::views::TextView;

pub fn run() {
    let mut zinc = cursive::default();
    zinc.add_global_callback('q', |z| z.quit());
    zinc.add_layer(TextView::new("hi goobabs"));
    zinc.run();
}