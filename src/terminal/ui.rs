pub fn run() {
    let mut zinc = cursive::default();
    zinc.add_global_callback('q', |z| z.quit());
}