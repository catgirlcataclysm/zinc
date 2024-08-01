use dircpy::copy_dir;
use log::{debug, error};
use std::{
    fs::{self, create_dir_all},
    io::Write,
    process::{exit, Command, Output},
};

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
    pub init: Init,
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

        self.set_offset();
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

        self.finalize_install();
        self.create_users();
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
                eprintln!("Your device has an unsupported baseboard. Considering this is booting, please reach out to me so I can look into it further.");
                exit(1);
            }
        }
        error!("Offset is {}", self.offset);
    }

    fn cgpt_tomfoolery(&self) {
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

        let output = Command::new("parted")
            .args(["--script", self.emmc.as_str(), "mklabel", "gpt"])
            .output()
            .expect("Failed to create GPT partition table.");
        debug_output(output);

        let output = Command::new("cgpt")
            .args(["create", self.emmc.as_str()])
            .output()
            .expect("Failed to create partition table on drive.");
        debug_output(output);

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

        let output = Command::new("cgpt")
            .args(["show", self.emmc.as_str()])
            .output()
            .expect("Failed to query remaining space to partition");
        debug_output(output.clone());

        let stdout = String::from_utf8(output.stdout).expect("Output has non UTF-8 characters!");

        let mut stdout_split = stdout.split_terminator('\n');

        let remaining_size: usize = stdout_split
            .find(|o| o.contains("Sec GPT table"))
            .expect("can't find 'Sec GPT table' in cgpt output")
            .split_whitespace()
            .nth(0)
            .expect("can't find remaining size")
            .parse()
            .expect("remaining size is not an integer");

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

        let output = Command::new("chroot")
            .args([
                "/mnt",
                "apt",
                "install",
                "-y",
                "u-boot-tools",
                "vboot-utils",
                "cgpt",
            ])
            .output()
            .expect("Failed to install necessary bootloader packages.");
        debug_output(output);
    }

    fn setup_void(&self) {}

    fn setup_voidmusl(&self) {}

    fn setup_gentoo(&self) {}

    fn finalize_install(&self) {
        let kver_raw = String::from_utf8(
            Command::new("uname")
                .arg("-r")
                .output()
                .expect("Failed to run 'uname -r'.")
                .stdout,
        )
        .unwrap();
        let kver = kver_raw.trim();

        create_dir_all("/mnt/CdFiles").expect("Failed to create /mnt/CdFiles.");
        copy_dir("/CdFiles", "/mnt/CdFiles")
            .expect("Failed to recursively copy /CdFiles to chroot.");
        create_dir_all("/mnt/lib/firmware").expect("Failed to create /mnt/lib/firmware.");
        copy_dir("/lib/firmware", "/mnt/lib/firmware")
            .expect("Failed to recursively copy /lib/firmware to /mnt/lib/firmware.");
        create_dir_all("/mnt/lib/modules").expect("Failed to create /mnt/lib/modules.");
        copy_dir(
            format!("/lib/modules/{}", kver),
            format!("/mnt/lib/modules/{}", kver),
        )
        .expect("Failed to recursively copy kernel modules to /mnt/lib/modules");

        match self.board {
            Board::Bob => {
                create_dir_all("/mnt/etc/udev/hwdb.d")
                    .expect("Failed to create /mnt/etc/udev/hwdb.d");
                fs::copy("/CdFiles/board/bob/accel-matrix.hwdb", "/mnt/etc/udev/hwdb.d/accel-matrix.hwdb").expect("Failed to copy accel-matrix.hwdb from cadmium board folder to /etc/udev/hwdb.d.");
                let output = Command::new("chroot")
                    .args(["/mnt", "udevadm", "hwdb", "-u"])
                    .output()
                    .expect("Failed to run 'udevadm hwdb -u' inside chroot.");
                debug_output(output);
            }
            Board::Coachz => {}
            Board::Hana => {
                create_dir_all("/mnt/etc/udev/hwdb.d")
                    .expect("Failed to create /mnt/etc/udev/hwdb.d");
                fs::copy("/CdFiles/board/hana/accel-matrix.hwdb", "/mnt/etc/udev/hwdb.d/accel-matrix.hwdb").expect("Failed to copy accel-matrix.hwdb from cadmium board folder to /etc/udev/hwdb.d.");
                let output = Command::new("chroot")
                    .args(["/mnt", "udevadm", "hwdb", "-u"])
                    .output()
                    .expect("Failed to run 'udevadm hwdb -u' inside chroot.");
                debug_output(output);
            }
            Board::Homestar => {}
            Board::Kevin => {
                create_dir_all("/mnt/etc/udev/hwdb.d")
                    .expect("Failed to create /mnt/etc/udev/hwdb.d");
                fs::copy("/CdFiles/board/kevin/accel-matrix.hwdb", "/mnt/etc/udev/hwdb.d/accel-matrix.hwdb").expect("Failed to copy accel-matrix.hwdb from cadmium board folder to /etc/udev/hwdb.d.");
                let output = Command::new("chroot")
                    .args(["/mnt", "udevadm", "hwdb", "-u"])
                    .output()
                    .expect("Failed to run 'udevadm hwdb -u' inside chroot.");
                debug_output(output);
            }
            Board::Kodama => {
                create_dir_all("/mnt/etc/libinput").expect("Failed to create /mnt/etc/libinput");
                fs::copy("/CdFiles/board/kodama/local-overrides.quirks", "/mnt/etc/libinput/local-overrides.quirks").expect("Failed to copy local-overrides.quirks from cadmium board folder to /etc/libinput.");
                create_dir_all("/mnt/etc/udev/hwdb.d")
                    .expect("Failed to create /mnt/etc/udev/hwdb.d");
                fs::copy("/CdFiles/board/kodama/accel-matrix.hwdb", "/mnt/etc/udev/hwdb.d/accel-matrix.hwdb").expect("Failed to copy accel-matrix.hwdb from cadmium board folder to /etc/udev/hwdb.d.");
                let output = Command::new("chroot")
                    .args(["/mnt", "udevadm", "hwdb", "-u"])
                    .output()
                    .expect("Failed to run 'udevadm hwdb -u' inside chroot.");
                debug_output(output);
            }
            Board::Krane => {
                create_dir_all("/mnt/etc/libinput").expect("Failed to create /mnt/etc/libinput");
                fs::copy("/CdFiles/board/krane/local-overrides.quirks", "/mnt/etc/libinput/local-overrides.quirks").expect("Failed to copy local-overrides.quirks from cadmium board folder to /etc/libinput.");
                create_dir_all("/mnt/etc/udev/hwdb.d")
                    .expect("Failed to create /mnt/etc/udev/hwdb.d");
                fs::copy("/CdFiles/board/krane/accel-matrix.hwdb", "/mnt/etc/udev/hwdb.d/accel-matrix.hwdb").expect("Failed to copy accel-matrix.hwdb from cadmium board folder to /etc/udev/hwdb.d.");
                let output = Command::new("chroot")
                    .args(["/mnt", "udevadm", "hwdb", "-u"])
                    .output()
                    .expect("Failed to run 'udevadm hwdb -u' inside chroot.");
                debug_output(output);
            }
            Board::Lazor => {}
            Board::Minnie => {
                create_dir_all("/mnt/etc/udev/hwdb.d")
                    .expect("Failed to create /mnt/etc/udev/hwdb.d");
                fs::copy("/CdFiles/board/minnie/accel-matrix.hwdb", "/mnt/etc/udev/hwdb.d/accel-matrix.hwdb").expect("Failed to copy accel-matrix.hwdb from cadmium board folder to /etc/udev/hwdb.d.");
                let output = Command::new("chroot")
                    .args(["/mnt", "udevadm", "hwdb", "-u"])
                    .output()
                    .expect("Failed to run 'udevadm hwdb -u' inside chroot.");
                debug_output(output);
            }
            Board::Speedy => {}
            Board::None => {}
        }

        if self.baseboard == Baseboard::Trogdor {
            let output = Command::new("make")
                .args(["-C", "/CdFiles/qmic", "prefix=/mnt/usr", "install"])
                .output()
                .expect("Failed to run make in /CdFiles/qmic.");
            debug_output(output);
            let output = Command::new("make")
                .args(["-C", "/CdFiles/qrtr", "prefix=/mnt/usr", "install"])
                .output()
                .expect("Failed to run make in /CdFiles/qrtr");
            debug_output(output);
            let output = Command::new("make")
                .args(["-C", "/CdFiles/rmtfs", "prefix=/mnt/usr", "install"])
                .output()
                .expect("Failed to run make in /CdFiles/rmtfs");
            debug_output(output);
        }
        match self.init {
            Init::Systemd => {
                let output = Command::new("chroot")
                    .args(["/mnt", "systemctl", "enable", "rmtfs"])
                    .output()
                    .expect("Failed to enable rmtfs service in chroot");
                debug_output(output);
            }
            Init::Openrc => {
                let output = Command::new("chroot")
                    .args(["/mnt", "rc-update", "add", "rmtfs", "default"])
                    .output()
                    .expect("Failed to enable rmtfs service in chroot");
                debug_output(output);
            }
            Init::Runit => {
                let output = Command::new("chroot")
                    .args(["/mnt", "sv", "up", "rmtfs"])
                    .output()
                    .expect("Failed to enable rmtfs service in chroot");
                debug_output(output);
            }
        }
        // tf why isnt this working, the logs show it working but the chromebook isnt bootable unless i do it manually
        let output = Command::new("dd")
            .args([
                "if=/dev/disk/by-partlabel/SDKernelA",
                "of=/dev/disk/by-partlabel/MMCKernelA",
            ])
            .output()
            .expect("Failed to copy Kernel to eMMC.");
        debug_output(output);
    }

    fn create_users(self) {
        match self.distro {
            Distro::ArchLinux => todo!(),
            Distro::Debian => {
                let output = Command::new("chroot")
                    .args(["/mnt", "/sbin/useradd", "-m", self.username.trim()])
                    .output()
                    .expect("Failed to create user in chroot.");
                debug_output(output);
                // need to input password
                let mut child = Command::new("chroot")
                    .args(["/mnt", "passwd", self.username.trim()])
                    .spawn()
                    .expect("Failed to set user password.");
                let mut child_stdin = child.stdin.as_ref().unwrap();
                child_stdin
                    .write_all(self.passwd.as_bytes())
                    .expect("Failed to write passwd to stdin.");
                child_stdin.flush().expect("Failed to flush stdin.");
                child.wait().expect("Failed to set user password.");
                // need to input root password
                let mut child = Command::new("chroot")
                    .args(["/mnt", "passwd"])
                    .spawn()
                    .expect("Failed to set root password.");
                let mut child_stdin = child.stdin.as_ref().unwrap();
                child_stdin
                    .write_all(self.rootpasswd.as_bytes())
                    .expect("Failed to write root passwd to stdin.");
                child_stdin.flush().expect("Failed to flush stdin.");
                child.wait().expect("Failed to set root password.");
            }
            Distro::Void => todo!(),
            Distro::VoidMusl => todo!(),
            Distro::Gentoo => todo!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Desktop {
    Kde,
    Gnome,
    Sway,
    Xfce,
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

        let output = Command::new("bash")
            .args(["-c", "lsof | { head -1 ; grep mmcblk ; }"])
            .output()
            .expect("yeag");
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
                    .unwrap_or_else(|_| {
                        panic!("Failed to create Btrfs filesystem on {}", rootpart)
                    });

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

#[derive(PartialEq)]
pub enum Init {
    Systemd,
    Openrc,
    Runit,
}

impl From<Distro> for Init {
    fn from(value: Distro) -> Self {
        match value {
            Distro::ArchLinux => Init::Systemd,
            Distro::Debian => Init::Systemd,
            Distro::Void => Init::Runit,
            Distro::VoidMusl => Init::Runit,
            Distro::Gentoo => Init::Openrc,
        }
    }
}

impl Default for Init {
    fn default() -> Self {
        Self::Systemd
    }
}

pub fn debug_output(output: Output) {
    debug!(
        "status: {}\nstdout: {}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
