use cursive::{views::{Button, LinearLayout, NamedView, PaddedView, Panel, RadioButton, TextArea, TextView}, Cursive};
use cursive_tabs::TabPanel;

enum Desktop {
    KDE,
    GNOME,
    Sway,
    XFCE
}

enum Filesystem {
    F2FS,
    Ext4,
    Btrfs
}

enum Distro {
    ArchLinux,
    Debian,
    Void,
    VoidMusl
}

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
            .child(PaddedView::lrtb(0, 9, 1, 3, Button::new("Begin", installer))))).title("Welcome!"));
    zinc.run();
}

fn installer(z: &mut Cursive) {

    let tabs = TabPanel::new()
        .with_tab(NamedView::new("Distro", PaddedView::lrtb(2, 2, 2, 2, LinearLayout::vertical()
            .child(RadioButton::global("distro", Distro::ArchLinux, "Arch Linux"))
            .child(RadioButton::global("distro", Distro::Debian, "Debian"))
            .child(RadioButton::global("distro", Distro::Void, "Void Linux"))
            .child(RadioButton::global("distro", Distro::VoidMusl, "Void Musl")))))
        .with_tab(NamedView::new("Filesystem", PaddedView::lrtb(2, 2, 2, 2, LinearLayout::vertical()
            .child(RadioButton::global("fs", Filesystem::F2FS, "F2FS"))
            .child(RadioButton::global("fs", Filesystem::Ext4, "Ext4"))
            .child(RadioButton::global("fs", Filesystem::Btrfs, "Btrfs")))))
        .with_tab(NamedView::new("Desktop", PaddedView::lrtb(2, 2, 2, 2, LinearLayout::vertical()
            .child(RadioButton::global("desktop", Desktop::KDE, "KDE Plasma"))
            .child(RadioButton::global("desktop", Desktop::GNOME, "Gnome"))
            .child(RadioButton::global("desktop", Desktop::Sway, "Sway"))
            .child(RadioButton::global("desktop", Desktop::XFCE, "XFCE")))))
        .with_tab(NamedView::new("Accounts", PaddedView::lrtb(2, 2, 2, 2, LinearLayout::vertical()
            .child(TextView::new("Enter your Username:"))
            .child(TextArea::new())
            .child(TextView::new("Enter your Password:"))
            .child(TextArea::new()))));

    z.pop_layer();
    z.add_layer(tabs);
}