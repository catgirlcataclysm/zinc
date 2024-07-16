use cursive::{views::{Button, EditView, LinearLayout, NamedView, PaddedView, Panel, RadioButton, TextView}, Cursive};
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
    VoidMusl,
    Gentoo
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
            .child(PaddedView::lrtb(0, 9, 1, 3, Button::new("Begin", config))))).title("Welcome!"));
    zinc.run();
}

fn config(z: &mut Cursive) {

    let tabs = TabPanel::new()
        .with_tab(NamedView::new("Distro", PaddedView::lrtb(2, 2, 2, 2, LinearLayout::vertical()
            .child(RadioButton::global("distro", Distro::ArchLinux, "Arch Linux"))
            .child(RadioButton::global("distro", Distro::Debian, "Debian"))
            .child(RadioButton::global("distro", Distro::Void, "Void Linux"))
            .child(RadioButton::global("distro", Distro::VoidMusl, "Void Musl"))
            .child(RadioButton::global("distro", Distro::Gentoo, "Gentoo")))))
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
            .child(EditView::new())
            .child(TextView::new("Enter your Password:"))
            .child(EditView::new())
            .child(PaddedView::lrtb(10, 0, 0, 0, Button::new("Finish", install))))));
            

    z.pop_layer();
    z.add_layer(tabs);
}

fn install(z: &mut Cursive) {

}