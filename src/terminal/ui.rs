use std::process::Command;

use cursive::{views::{Button, LinearLayout, PaddedView, Panel, TextView}, Cursive};

pub fn run() {
    let mut zinc = cursive::default();

let logo = TextView::new(
    r"     $$$$$$$$\           
    \____$$  |          
        $$  / $$$$$$$\  
       $$  /  $$  __$$\ 
      $$  /   $$ |  $$ |
     $$  /    $$ |  $$ |
    $$$$$$$$\ $$ |  $$ |
    \________|\__|  \__|
    ");

    zinc.add_layer(Panel::new(LinearLayout::horizontal()
        .child(PaddedView::lrtb(2, 2, 2, 2, logo))
        .child(LinearLayout::vertical()
            .child(PaddedView::lrtb(2, 2, 6, 2, TextView::new("Welcome to Zinc, the guided installer for Cadmium Linux!")))
            .child(PaddedView::lrtb(0, 9, 1, 3, Button::new("Begin", choose_fs))))).title("Welcome!"));
    zinc.run();
}

fn choose_fs(z: &mut Cursive) {
    z.pop_layer();
}