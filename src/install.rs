use std::{fs::create_dir_all, process::{exit, Command, Output}};
use dircpy::copy_dir;
use log::error;

use crate::hardware::{Baseboard, Board};

#[derive(Default)]
pub struct Install {
    pub baseboard: Baseboard,
    pub board: Board,
    pub emmc: String,
    pub distro: Distro,
    pub fs: Filesystem,
    pub desktop: Desktop,
    pub rootpasswd: String,
    pub username: String,
    pub passwd: String,
    pub offset: usize,
}

impl Install {
    pub fn start(mut self) {
        //error!("Installing dependencies");
        //let output = Command::new("apk")
        //    .args([
        //        "add",
        //        "u-boot-tools",
        //        "vboot-utils",
        //        "btrfs-progs",
        //        "f2fs-tools",
        //        "git",
        //        "networkmanager",
        //        "sudo",
        //        "vim",
        //        "cgpt",
        //        "parted",
        //        "wget",
        //    ])
        //    .output()
        //    .expect("Failed to install necessary installation dependencies.");
        //debug_output(output);

        error!("Setting offset");
        self.set_offset();
        error!("Setting up partitions with cgpt");
        self.cgpt_tomfoolery();
        self.fs.mkfs();

        match self.distro {
            Distro::ArchLinux => {
                self.setup_archlinux();
            }
            Distro::Debian => {
                self.setup_debian();
            }
            Distro::Void => {
                self.setup_void();
            }
            Distro::VoidMusl => {
                self.setup_voidmusl();
            }
            Distro::Gentoo => {
                self.setup_gentoo();
            }
        }

        self.finalise();
    }

    pub fn set_offset(&mut self) {    
        match self.baseboard {
            Baseboard::Gru => {
                self.offset = 0;
            }
            Baseboard::Kukui => {
                self.offset = 0;
            }
            Baseboard::Oak => {
                self.offset = 0;
            }
            Baseboard::Trogdor => {
                self.offset = 0;
            }
            Baseboard::Veyron => {
                self.offset = 16384;
            }
            Baseboard::None => {
                eprintln!("ya fucked up somehow this should be an unreachable error, ask for support on github or discord lol");
                exit(1);
            }
        }
        error!("Offset is {}", self.offset);
    }

    fn cgpt_tomfoolery(&self) {
        error!("Running dd");
        let output = Command::new("dd")
            .args([
                "if=/dev/zero",
                format!("of={}", self.emmc).as_str(),
                "bs=512k",
                "count=128",
                format!("seek={}", self.offset).as_str(),
            ])
            .output()
            .expect("Failed to zero beginning of the drive.");
        debug_output(output);
    
        error!("Running parted");
        let output = Command::new("parted")
            .args(["--script", self.emmc.as_str(), "mklabel", "gpt"])
            .output()
            .expect("Failed to create GPT partition table.");
        debug_output(output);

        //fails and idk why
        // Should not fail anymore, needs testing - Radical
        // havent experienced a fail, lets see if it comes back - k
        error!("Running cgpt create");
        let output = Command::new("cgpt")
            .args(["create", self.emmc.as_str()])
            .output()
            .expect("Failed to create partition table on drive.");
        debug_output(output);
    
        error!("Running cgpt add MMCKernelA");
        let output = Command::new("cgpt")
            .args([
                "add",
                "-i",
                "1",
                "-t",
                "kernel",
                "-b",
                (8192 + self.offset).to_string().as_str(),
                "-s",
                "65536",
                "-l",
                "MMCKernelA",
                "-S",
                "1",
                "-T",
                "2",
                "-P",
                "10",
                self.emmc.as_str(),
            ])
            .output()
            .expect("Failed to add first partition to eMMC.");
        debug_output(output);
    
        error!("Running cgpt add MMCKernelB");
        let output = Command::new("cgpt")
            .args([
                "add",
                "-i",
                "2",
                "-t",
                "kernel",
                "-b",
                (73728 + self.offset).to_string().as_str(),
                "-s",
                "65536",
                "-l",
                "MMCKernelB",
                "-S",
                "0",
                "-T",
                "2",
                "-P",
                "5",
                self.emmc.as_str(),
            ])
            .output()
            .expect("Failed to add second partition to eMMC.");
        debug_output(output);

        error!("Getting remaining size");
        let output = Command::new("cgpt")
            .args([
                "show",
                self.emmc.as_str(),
            ])
            .output()
            .expect("Failed to query remaining space to partition");
        debug_output(output.clone());

        let stdout = String::from_utf8(output.stdout).expect("Output has non UTF-8 characters!");

        let mut stdout_split = stdout.split_terminator("\n");
        error!("split: {:#?}", stdout_split);
        

        // subtract overflow error need to fix
        let remaining_size: usize = stdout_split
            .find(|o| o.contains("Sec GPT table"))
            .expect("can't find 'Sec GPT table' in cgpt output")
            .split_whitespace()
            .nth(0)
            .expect("can't find remaining size")
            .parse()
            .expect("remaining size is not an integer");
        error!("Remaining size: {}", remaining_size);
    
        error!("Running cgpt add data");
        let output = Command::new("cgpt")
            .args([
                "add",
                "-i",
                "3",
                "-t",
                "data",
                "-b",
                (139264 + self.offset).to_string().as_str(),
                "-s",
                (remaining_size - (139264 + self.offset))
                    .to_string()
                    .as_str(),
                "-l",
                "Root",
                self.emmc.as_str(),
            ])
            .output()
            .expect("Failed to add final partition to eMMC.");
        debug_output(output);
    }

    fn setup_archlinux(&self) {}

    fn setup_debian(&self) {
        #[cfg(target_pointer_width = "64")]
        let output = Command::new("debootstrap")
            .args([
                "--arch=arm64",
                "bookworm",
                "/mnt",
                "https://deb.debian.org/debian/",
            ])
            .output()
            .expect("Failed to run debootstrap.");
        #[cfg(target_pointer_width = "32")]
        let output = Command::new("debootstrap")
            .args([
                "--arch=armhf",
                "bookworm",
                "/mnt",
                "https://deb.debian.org/debian/",
            ])
            .output()
            .expect("Failed to run debootstrap.");
        debug_output(output);

        let output = Command::new("chroot")
            .args(["/mnt", "apt", "update"])
            .output()
            .expect("Failed to run apt update inside chroot.");
        debug_output(output);
    }

    fn setup_void(&self) {}

    fn setup_voidmusl(&self) {}

    fn setup_gentoo(&self) {}

    fn finalise(&self) {
        create_dir_all("/mnt/CdFiles").expect("Failed to create /mnt/CdFiles.");
        copy_dir("/CdFiles", "/mnt/CdFiles")
            .expect("Failed to recursively copy /CdFiles to chroot.");
        create_dir_all("/mnt/lib/firmware").expect("Failed to create /mnt/lib/firmware.");
        copy_dir("/lib/firmware", "/mnt/lib/firmware")
            .expect("Failed to recursively copy /lib/firmware to /mnt/lib/firmware.");
        create_dir_all("/mnt/lib/modules").expect("Failed to create /mnt/lib/modules.");
        copy_dir(
            format!(
                "/lib/modules/{}",
                String::from_utf8(
                    Command::new("uname")
                        .arg("-r")
                        .output()
                        .expect("Failed to run 'uname -r'.")
                        .stdout
                )
                .unwrap()
            ),
            format!(
                "/mnt/lib/modules/{}",
                String::from_utf8(
                    Command::new("uname")
                        .arg("-r")
                        .output()
                        .expect("Failed to run 'uname -r'.")
                        .stdout
                )
                .unwrap()
            )
        )
        .expect("Failed to recursively copy kernel modules to /mnt/lib/modules");
    }

}

#[derive(Clone, Copy, Debug)]
pub enum Desktop {
    KDE,
    GNOME,
    Sway,
    XFCE,
    None,
}

impl Default for Desktop {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Filesystem {
    F2FS,
    Ext4,
    Btrfs,
}

impl Default for Filesystem {
    fn default() -> Self {
        Self::F2FS
    }
}

impl Filesystem {
    fn mkfs(&self) {
        let rootpart = "/dev/disk/by-partlabel/Root";
        
        let output = Command::new("bash").args(["-c", "lsof | { head -1 ; grep mmcblk ; }"]).output().expect("yeag");
        debug_output(output);
        
        match self {
            Self::F2FS => {
                let output = Command::new("mkfs.f2fs")
                    .args(["-f", rootpart])
                    .output()
                    .unwrap_or_else(|_| panic!("Failed to create F2FS filesystem on {}", rootpart));
                debug_output(output);

                let output = Command::new("mount")
                    .args([
                        "-o",
                        "compress_algorithm=zstd:6,compress_chksum,atgc,gc_merge,lazytime",
                        rootpart,
                        "/mnt",
                    ])
                    .output()
                    .expect("Failed to mount F2FS filesystem to /mnt");
                debug_output(output);
            }
            Self::Ext4 => {
                Command::new("mkfs.ext4")
                    .args(["-F", rootpart])
                    .output()
                    .unwrap_or_else(|_| panic!("Failed to create Ext4 filesystem on {}", rootpart));
    
                Command::new("mount")
                    .args([rootpart, "/mnt"])
                    .output()
                    .expect("Failed to mount Ext4 filesystem to /mnt");
            }
            Self::Btrfs => {
                Command::new("mkfs.btrfs")
                    .args(["-f", rootpart])
                    .output()
                    .unwrap_or_else(|_| panic!("Failed to create Btrfs filesystem on {}", rootpart));
    
                Command::new("mount")
                    .args([rootpart, "/mnt"])
                    .output()
                    .expect("Failed to mount Btrfs filesystem to /mnt.");
    
                Command::new("btrfs")
                    .args(["subvolume", "create", "/mnt/.system"])
                    .output()
                    .expect("Failed to create system subvolume.");
    
                Command::new("btrfs")
                    .args(["subvolume", "create", "/mnt/.system/root"])
                    .output()
                    .expect("Failed to create root subvolume.");
    
                Command::new("btrfs")
                    .args(["subvolume", "create", "/mnt/.system/home"])
                    .output()
                    .expect("Failed to create home subvolume.");
    
                Command::new("btrfs")
                    .args(["subvolume", "create", "/mnt/.snapshots"])
                    .output()
                    .expect("Failed to create snapshots subvolume.");
    
                Command::new("umount")
                    .arg("/mnt")
                    .output()
                    .expect("Failed to unmount btrfs filesystem.");
    
                Command::new("mount")
                    .args([
                        "-o",
                        "compress=zstd:6,subvol=.system/root",
                        rootpart,
                        "/mnt",
                    ])
                    .output()
                    .expect("Failed to mount root subvolume to /mnt");
    
                Command::new("mount")
                    .args([
                        "--mkdir",
                        "-o",
                        "compress=zstd:6,subvol=.system/home",
                        rootpart,
                        "/mnt/home",
                    ])
                    .output()
                    .expect("Failed to mount home subvolume to /mnt/home");
    
                Command::new("mount")
                    .args([
                        "--mkdir",
                        "-o",
                        "compress=zstd:6,subvol=.snapshots",
                        rootpart,
                        "/mnt/.snapshots",
                    ])
                    .output()
                    .expect("Failed to mount snapshots subvolume to /mnt/.snapshots");
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Distro {
    ArchLinux,
    Debian,
    Void,
    VoidMusl,
    Gentoo,
}

impl Default for Distro {
    fn default() -> Self {
        Self::ArchLinux
    }
}

fn debug_output(output: Output) {
    error!(
        "status: {}\nstdout: {}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
