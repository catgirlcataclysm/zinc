use std::fs::{self, read_dir};
use cursive::{view::{Nameable, Resizable}, views::{Button, EditView, LinearLayout, NamedView, PaddedView, Panel, RadioButton, RadioGroup, TextView}, Cursive};
use cursive_tabs::TabPanel;
use crate::{hardware::{Baseboard, Board}, install, BASEBOARDS, BOARDS};



pub struct Selections {
    pub baseboard: Baseboard,
    pub board: Board,
    pub emmc: String,
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
            .child(PaddedView::lrtb(0, 9, 1, 3, Button::new("Begin", get_hardware))))).title("Welcome!"));
    zinc.run();
}

fn get_hardware(z: &mut Cursive) {

    let hardware_raw = fs::read_to_string("/sys/firmware/devicetree/base/compatible").expect("Failed to get board info.");
    let board: Board = BOARDS.iter().find(|b| hardware_raw.contains(*b)).expect("Your board isnt supported.").into();

    let baseboard: Baseboard = match board {
        Board::Bob => Baseboard::Gru,
        Board::Coachz => Baseboard::Trogdor,
        Board::Hana => Baseboard::Oak,
        Board::Homestar => Baseboard::Trogdor,
        Board::Kevin => Baseboard::Gru,
        Board::Kodama => Baseboard::Kukui,
        Board::Krane => Baseboard::Kukui,
        Board::Lazor => Baseboard::Trogdor,
        Board::Minnie => Baseboard::Veyron,
        Board::Speedy => Baseboard::Veyron,
        Board::None => Baseboard::None
    };

    //let dev = read_dir("/dev").expect("Failed to get list of storage devices.");
    //for path in dev {
    //    if let Ok(path) = path {
    //        if !path.path().to_string_lossy().contains("/dev/mmcblk") {
    //            continue;
    //        }
    //        //can contain boot1, rpmb, and any partition number(/dev/mmcblkp*) im stupid
    //        if path.path().to_string_lossy().contains("boot0") {
    //            let emmc = path.path().to_string_lossy().replace("boot0", "");
    //            config(z, emmc, board.clone(), baseboard.clone());
    //        } else {
    //            let emmc = path.path().to_string_lossy().into_owned();
    //            config(z, emmc, board.clone(), baseboard.clone());
    //        }
    //    }
    //}
    let emmc = "/dev/mmcblk0".to_string();
    config(z, emmc, board.clone(), baseboard.clone());
}


fn config(z: &mut Cursive, emmc: String, board: Board, baseboard: Baseboard) {

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
            .child(EditView::new().with_name("rootpasswd").fixed_height(1))
            .child(TextView::new("Enter your Username:"))
            .child(EditView::new().with_name("username").fixed_height(1))
            .child(TextView::new("Enter your Password:"))
            .child(EditView::new().with_name("passwd").fixed_height(1))
            .child(PaddedView::lrtb(10, 0, 0, 0, Button::new("Finish", move |z| finish(z, emmc.clone(), board.clone(), baseboard.clone())))))));
            

    z.pop_layer();
    z.add_layer(tabs);
}

fn finish(z: &mut Cursive, emmc: String, board: Board, baseboard: Baseboard) {
    // TODO: handle user not inputting username and password
    let distro = *RadioGroup::<Distro>::with_global("distro", |distro| distro.selection().clone());
    let fs = *RadioGroup::<Filesystem>::with_global("fs", |fs| fs.selection().clone());
    let desktop = *RadioGroup::<Desktop>::with_global("desktop", |de| de.selection().clone());
    let username = z.call_on_name("username", |view: &mut EditView| view.get_content()).unwrap().to_string();
    let passwd = z.call_on_name("passwd", |view: &mut EditView| view.get_content()).unwrap().to_string();
    let rootpasswd = z.call_on_name("rootpasswd", |view: &mut EditView| view.get_content()).unwrap().to_string();

    let selection = Selections {
        baseboard,
        board,
        emmc,
        distro,
        fs,
        desktop,
        rootpasswd,
        username,
        passwd
    };

    z.pop_layer();

   install::begin_install(selection);
}