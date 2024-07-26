use crate::{
    hardware::{self, Board},
    install::{Desktop, Distro, Filesystem, Install},
};
use cursive::{
    view::{Nameable, Resizable},
    views::{
        Button, EditView, LinearLayout, NamedView, PaddedView, Panel, RadioButton, RadioGroup,
        TextView,
    },
    Cursive,
};
use cursive_tabs::TabPanel;

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
    ",
    );

    zinc.add_layer(
        Panel::new(
            LinearLayout::horizontal()
                .child(PaddedView::lrtb(2, 2, 2, 2, logo))
                .child(
                    LinearLayout::vertical()
                        .child(PaddedView::lrtb(
                            2,
                            2,
                            6,
                            2,
                            TextView::new(
                                "Welcome to Zinc, the guided installer for Cadmium Linux!",
                            ),
                        ))
                        .child(PaddedView::lrtb(0, 9, 1, 3, Button::new("Begin", config))),
                ),
        )
        .title("Welcome!"),
    );
    zinc.run();
}

fn config(z: &mut Cursive) {
    let tabs = TabPanel::new()
        .with_tab(NamedView::new(
            "Distro",
            PaddedView::lrtb(
                2,
                2,
                2,
                2,
                LinearLayout::vertical()
                    .child(RadioButton::global(
                        "distro",
                        Distro::ArchLinux,
                        "Arch Linux",
                    ))
                    .child(RadioButton::global("distro", Distro::Debian, "Debian"))
                    .child(RadioButton::global("distro", Distro::Void, "Void Linux"))
                    .child(RadioButton::global("distro", Distro::VoidMusl, "Void Musl"))
                    .child(RadioButton::global("distro", Distro::Gentoo, "Gentoo")),
            ),
        ))
        .with_tab(NamedView::new(
            "Filesystem",
            PaddedView::lrtb(
                2,
                2,
                2,
                2,
                LinearLayout::vertical()
                    .child(RadioButton::global("fs", Filesystem::F2FS, "F2FS"))
                    .child(RadioButton::global("fs", Filesystem::Ext4, "Ext4"))
                    .child(RadioButton::global("fs", Filesystem::Btrfs, "Btrfs")),
            ),
        ))
        .with_tab(NamedView::new(
            "Desktop",
            PaddedView::lrtb(
                2,
                2,
                2,
                2,
                LinearLayout::vertical()
                    .child(RadioButton::global("desktop", Desktop::KDE, "KDE Plasma"))
                    .child(RadioButton::global("desktop", Desktop::GNOME, "Gnome"))
                    .child(RadioButton::global("desktop", Desktop::Sway, "Sway"))
                    .child(RadioButton::global("desktop", Desktop::XFCE, "XFCE")),
            ),
        ))
        .with_tab(NamedView::new(
            "Accounts",
            PaddedView::lrtb(
                2,
                2,
                2,
                2,
                LinearLayout::vertical()
                    .child(TextView::new("Enter the root password:"))
                    .child(EditView::new().with_name("rootpasswd").fixed_height(1))
                    .child(TextView::new("Enter your Username:"))
                    .child(EditView::new().with_name("username").fixed_height(1))
                    .child(TextView::new("Enter your Password:"))
                    .child(EditView::new().with_name("passwd").fixed_height(1))
                    .child(PaddedView::lrtb(10, 0, 0, 0, Button::new("Finish", finish))),
            ),
        ));

    z.pop_layer();
    z.add_layer(tabs);
}

fn finish(z: &mut Cursive) {
    let board = Board::get();
    let emmc = hardware::get_emmc().expect("Where the fork is your eMMC?");

    // TODO: handle user not inputting username and password
    let distro = *RadioGroup::<Distro>::with_global("distro", |distro| distro.selection().clone());
    let fs = *RadioGroup::<Filesystem>::with_global("fs", |fs| fs.selection().clone());
    let desktop = *RadioGroup::<Desktop>::with_global("desktop", |de| de.selection().clone());
    let username = z
        .call_on_name("username", |view: &mut EditView| view.get_content())
        .unwrap()
        .to_string();
    let passwd = z
        .call_on_name("passwd", |view: &mut EditView| view.get_content())
        .unwrap()
        .to_string();
    let rootpasswd = z
        .call_on_name("rootpasswd", |view: &mut EditView| view.get_content())
        .unwrap()
        .to_string();

    let selection = Install {
        baseboard: board.into(),
        board,
        emmc,
        distro,
        fs,
        desktop,
        rootpasswd,
        username,
        passwd,
        ..Default::default()
    };

    z.pop_layer();

    Install::start(selection);
}
