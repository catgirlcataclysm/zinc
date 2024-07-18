#[ path = "install.rs"] mod install;

use std::{fs::{read_dir, DirEntry, ReadDir}, io::Error};

use cursive::{view::{Nameable, Resizable}, views::{Button, EditView, LinearLayout, NamedView, PaddedView, Panel, RadioButton, RadioGroup, TextView}, Cursive};
use cursive_tabs::TabPanel;
pub struct Selections {
    pub device: String,
    pub distro: Distro,
    pub fs: Filesystem,
    pub desktop: Desktop,
    pub rootpasswd: String,
    pub username: String,
    pub passwd: String
}

#[derive(Clone, Copy, Debug)]
pub enum Desktop {
    KDE,
    GNOME,
    Sway,
    XFCE
}

#[derive(Clone, Copy, Debug)]
pub enum Filesystem {
    F2FS,
    Ext4,
    Btrfs
}

#[derive(Clone, Copy, Debug)]
pub enum Distro {
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
            .child(PaddedView::lrtb(0, 9, 1, 3, Button::new("Begin", find_devices))))).title("Welcome!"));
    zinc.run();
}

fn find_devices(_z: &mut Cursive) {
    let dev = read_dir("/dev").expect("Failed to get list of storage devices.");
    let paths: Result<Vec<DirEntry>, Error> = dev.into_iter().collect();


    }


//fn config(z: &mut Cursive) {
//
//    let tabs = TabPanel::new()
//        .with_tab(NamedView::new("Device", PaddedView::lrtb(2, 2, 2, 2, LinearLayout::vertical()
//            .child(RadioButton::global("device", , label)))))
//        .with_tab(NamedView::new("Distro", PaddedView::lrtb(2, 2, 2, 2, LinearLayout::vertical()
//            .child(RadioButton::global("distro", Distro::ArchLinux, "Arch Linux"))
//            .child(RadioButton::global("distro", Distro::Debian, "Debian"))
//            .child(RadioButton::global("distro", Distro::Void, "Void Linux"))
//            .child(RadioButton::global("distro", Distro::VoidMusl, "Void Musl"))
//            .child(RadioButton::global("distro", Distro::Gentoo, "Gentoo")))))
//        .with_tab(NamedView::new("Filesystem", PaddedView::lrtb(2, 2, 2, 2, LinearLayout::vertical()
//            .child(RadioButton::global("fs", Filesystem::F2FS, "F2FS"))
//            .child(RadioButton::global("fs", Filesystem::Ext4, "Ext4"))
//            .child(RadioButton::global("fs", Filesystem::Btrfs, "Btrfs")))))
//        .with_tab(NamedView::new("Desktop", PaddedView::lrtb(2, 2, 2, 2, LinearLayout::vertical()
//            .child(RadioButton::global("desktop", Desktop::KDE, "KDE Plasma"))
//            .child(RadioButton::global("desktop", Desktop::GNOME, "Gnome"))
//            .child(RadioButton::global("desktop", Desktop::Sway, "Sway"))
//            .child(RadioButton::global("desktop", Desktop::XFCE, "XFCE")))))
//        .with_tab(NamedView::new("Accounts", PaddedView::lrtb(2, 2, 2, 2, LinearLayout::vertical()
//            .child(TextView::new("Enter the root password:"))
//            .child(EditView::new().with_name("rootpasswd").fixed_height(1))
//            .child(TextView::new("Enter your Username:"))
//            .child(EditView::new().with_name("username").fixed_height(1))
//            .child(TextView::new("Enter your Password:"))
//            .child(EditView::new().with_name("passwd").fixed_height(1))
//            .child(PaddedView::lrtb(10, 0, 0, 0, Button::new("Finish", finish))))));
//            
//
//    z.pop_layer();
//    z.add_layer(tabs);
//}
//
//fn finish(z: &mut Cursive) {
//    // TODO: handle user not inputting username and password
//    let distro = *RadioGroup::<Distro>::with_global("distro", |distro| distro.selection().clone());
//    let fs = *RadioGroup::<Filesystem>::with_global("fs", |fs| fs.selection().clone());
//    let desktop = *RadioGroup::<Desktop>::with_global("desktop", |de| de.selection().clone());
//    let username = z.call_on_name("username", |view: &mut EditView| view.get_content()).unwrap().to_string();
//    let passwd = z.call_on_name("passwd", |view: &mut EditView| view.get_content()).unwrap().to_string();
//    let rootpasswd = z.call_on_name("rootpasswd", |view: &mut EditView| view.get_content()).unwrap().to_string();
//
//    let selection = Selections {
//        device,
//        distro,
//        fs,
//        desktop,
//        rootpasswd,
//        username,
//        passwd
//    };
//
//    z.pop_layer();
//
//    install::cgpt_tomfoolery(selection);
//}