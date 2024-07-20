use std::process::Command;

use crate::{hardware::{Board, Baseboard}, zinc::{Distro, Filesystem, Selections}};
pub fn begin_install(sels: Selections) {

    Command::new("apk")
        .args(["add", "u-boot-tools", "vboot-utils", "btrfs-progs", "f2fs-tools", "git", "networkmanager", "sudo", "vim", "cgpt", "parted", "wget"])
        .spawn()
        .expect("Failed to install necessary installation dependencies.");
    
    match sels.baseboard {
        Baseboard::Gru => {
            let offset = 0;
            cgpt_tomfoolery(offset, sels)
        }
        Baseboard::Kukui => {
            let offset = 0;
            cgpt_tomfoolery(offset, sels)
        }
        Baseboard::Oak => {
            let offset = 0;
            cgpt_tomfoolery(offset, sels)
        }
        Baseboard::Trogdor => {
            let offset = 0;
            cgpt_tomfoolery(offset, sels)
        }
        Baseboard::Veyron => {
            let offset = 16384;
            cgpt_tomfoolery(offset, sels)
        }
        Baseboard::None => {
            eprintln!("ya fucked up somehow this should be an unreachable error, ask for support on github or discord lol");
        }

    }
}

fn cgpt_tomfoolery(offset: u32, sels: Selections) {
    
    Command::new("dd").args(["if=/dev/zero", format!("of={}", sels.emmc).as_str(), "bs=512k", "count=128", format!("seek={}", offset).as_str()])
        .spawn()
        .expect("Failed to zero beginning of the drive.");
    
    Command::new("parted").args(["--script", sels.emmc.as_str(), "mklabel", "gpt"])
        .spawn()
        .expect("Failed to create GPT partition table.");
    
    Command::new("cgpt")
        .args(["create", sels.emmc.as_str()])
        .spawn()
        .expect("Failed to create partition table on drive.");

    Command::new("cgpt")
        .args(["add", "-i", "1", "-t", "kernel", "-b", (8192 + offset).to_string().as_str(),"-s", "65536", "-l", "MMCKernelA", "-S", "1", "-T", "2", "-P", "10", sels.emmc.as_str()])
        .spawn()
        .expect("Failed to add first partition to eMMC.");

    Command::new("cgpt")
        .args(["add", "-i", "2", "-t", "kernel", "-b", (73728 + offset).to_string().as_str(), "-s", "65536", "-l", "MMCKernelB", "-S", "0", "-T", "2", "-P", "5", sels.emmc.as_str()])
        .spawn()
        .expect("Failed to add second partition to eMMC.");
    
    let remaining_size: u64 = String::from_utf8(Command::new("cgpt").args(["show", sels.emmc.as_str(), "|", "grep", "'Sec GPT table'", "|", "awk", "'{print $1}'"])
        .output()
        .expect("Failed to query remaining space to partition").stdout)
        .unwrap()
        .parse().unwrap();

    Command::new("cgpt")
        .args(["add", "-i", "3", "-t", "data", "-b", (139264 + offset).to_string().as_str(), "-s", (remaining_size - (139264 + offset as u64)).to_string().as_str(), "-l", "Root", sels.emmc.as_str()])
        .spawn()
        .expect("Failed to add final partition to eMMC.");

    mkfs(sels)

}

fn mkfs(sels: Selections) {
    let rootpart = "/dev/disk/by-partlabel/Root";
    match sels.fs {
        Filesystem::F2FS => {
            Command::new("mkfs.f2fs")
                .args(["-f", rootpart])
                .spawn()
                .expect(format!("Failed to create F2FS filesystem on {}", rootpart).as_str());

            Command::new("mount")
                .args(["-o", "compress_algorithm=zstd:6,compress_chksum,atgc,gc_merge,lazytime", rootpart, "/mnt"])
                .spawn()
                .expect("Failed to mount F2FS filesystem to /mnt");
            
            match_distro(sels);
        }
        Filesystem::Ext4 => {
            Command::new("mkfs.ext4")
                .args(["-F", rootpart])
                .spawn()
                .expect(format!("Failed to create Ext4 Filesystem on {}", rootpart).as_str());

            Command::new("mount")
                .args([rootpart, "/mnt"])
                .spawn()
                .expect("Failed to mount Ext4 filesystem to /mnt");
            
            match_distro(sels);
        }
        Filesystem::Btrfs => {
            Command::new("mkfs.btrfs")
                .args(["-f", rootpart])
                .spawn()
                .expect(format!("Failed to create btrfs filesystem on {}", rootpart).as_str());
            
            Command::new("mount")
                .args([rootpart, "/mnt"])
                .spawn()
                .expect("Failed to mount Btrfs filesystem to /mnt.");
            
            Command::new("btrfs")
                .args(["subvolume", "create", "/mnt/.system"])
                .spawn()
                .expect("Failed to create system subvolume.");

            Command::new("btrfs")
                .args(["subvolume", "create", "/mnt/.system/root"])
                .spawn()
                .expect("Failed to create root subvolume.");
            
            Command::new("btrfs")
                .args(["subvolume", "create", "/mnt/.system/home"])
                .spawn()
                .expect("Failed to create home subvolume.");
            
            Command::new("btrfs")
                .args(["subvolume", "create", "/mnt/.snapshots"])
                .spawn()
                .expect("Failed to create snapshots subvolume.");
            
            Command::new("umount")
                .arg("/mnt")
                .spawn()
                .expect("Failed to unmount btrfs filesystem.");
            
            Command::new("mount")
                .args(["-o", "compress=zstd:6,subvol=.system/root", rootpart, "/mnt"])
                .spawn()
                .expect("Failed to mount root subvolume to /mnt");
            
            Command::new("mount")
                .args(["--mkdir", "-o", "compress=zstd:6,subvol=.system/home", rootpart, "/mnt/home"])
                .spawn()
                .expect("Failed to mount home subvolume to /mnt/home");
            
            Command::new("mount")
                .args(["--mkdir", "-o", "compress=zstd:6,subvol=.snapshots", rootpart, "/mnt/.snapshots"])
                .spawn()
                .expect("Failed to mount snapshots subvolume to /mnt/.snapshots");
            
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