#[ path = "install.rs"] mod install;

use cursive::{view::Resizable, views::{Button, Dialog, EditView, LinearLayout, NamedView, PaddedView, Panel, RadioButton, RadioGroup, TextView}, Cursive};
use cursive_tabs::TabPanel;

#[derive(Clone)]
pub struct Selections {
    pub distro: String,
    pub fs: String,
    pub desktop: String,
    pub rootpasswd: String,
    pub username: String,
    pub passwd: String
}

#[derive(Clone, Copy, Debug)]
enum Desktop {
    KDE,
    GNOME,
    Sway,
    XFCE
}

#[derive(Clone, Copy, Debug)]
enum Filesystem {
    F2FS,
    Ext4,
    Btrfs
}

#[derive(Clone, Copy, Debug)]
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
            .child(TextView::new("Enter the root password:"))
            .child(NamedView::new("rootpasswd", EditView::new().fixed_height(1)))
            .child(TextView::new("Enter your Username:"))
            .child(NamedView::new("username", EditView::new().fixed_height(1)))
            .child(TextView::new("Enter your Password:"))
            .child(NamedView::new("passwd", EditView::new().fixed_height(1)))
            .child(PaddedView::lrtb(10, 0, 0, 0, Button::new("Finish", finish))))));
            

    z.pop_layer();
    z.add_layer(tabs);
    
    fn finish(z: &mut Cursive) {
        // TODO: handle user not inputting username and password
        // TODO: fix all username and password shit in general, editview get content not working, always reports none
        let distro = format!("{:?}", RadioGroup::<Distro>::with_global("distro", |distro| distro.selection().clone()));
        let fs = format!("{:?}", RadioGroup::<Filesystem>::with_global("fs", |fs| fs.selection().clone()));
        let desktop = format!("{:?}", RadioGroup::<Desktop>::with_global("desktop", |de| de.selection().clone()));
        let username = z.call_on_name("username", |view: &mut EditView| view.get_content()).unwrap();
        let passwd = z.call_on_name("passwd", |view: &mut EditView| view.get_content()).unwrap();
        let rootpasswd = z.call_on_name("rootpasswd", |view: &mut EditView| view.get_content()).unwrap();

        let selection = Selections {
            distro,
            fs,
            desktop,
            rootpasswd: rootpasswd.to_string(),
            username: username.to_string(),
            passwd: passwd.to_string() 
        };

        z.pop_layer();
        
        {
            let selection = selection.clone();

            z.add_layer(Dialog::new().content(LinearLayout::vertical()
                .child(TextView::new(selection.distro))
                .child(TextView::new(selection.fs))
                .child(TextView::new(selection.desktop))
                .child(TextView::new(selection.rootpasswd))
                .child(TextView::new(selection.username))
                .child(TextView::new(selection.passwd))));
        }
        
        install::install(selection);
    }
}
