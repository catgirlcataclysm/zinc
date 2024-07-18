use crate::ui::{Selections, Distro, Filesystem};

pub fn cgpt_tomfoolery(sels: Selections) {
    
    mkfs(sels);
}

fn mkfs(sels: Selections) {
    match sels.fs {
        Filesystem::F2FS => {
            // mkfs.f2fs
            match_distro(sels);
        }
        Filesystem::Ext4 => {
            // mkfs.ext4
            match_distro(sels);
        }
        Filesystem::Btrfs => {
            //mkfs.btrfs
            match_distro(sels);
        }
    }    
}

fn match_distro(sels: Selections) { 
    match sels.distro {
        Distro::ArchLinux => {
            install_archlinux(sels);
        }
        Distro::Debian => {
            install_debian(sels);
        }
        Distro::Void => {
            install_void(sels);
        }
        Distro::VoidMusl => {
            install_voidmusl(sels);
        }
        Distro::Gentoo => {
            install_gentoo(sels);
        }
    }
}

fn install_archlinux(sels: Selections) {

}

fn install_debian(sels: Selections) {

}

fn install_void(sels: Selections) {

}

fn install_voidmusl(sels: Selections) {

}

fn install_gentoo(sels: Selections) {

}