use core::time::Duration;
use dircpy::copy_dir;
use log::{debug, error};
use reqwest::blocking::Client;
use std::{
    fs::{create_dir_all, remove_dir_all, OpenOptions},
    io::{copy, Write},
    process::{exit, Command, Output, Stdio},
    thread::sleep,
};

#[derive(Default)]
pub struct Install {
    pub emmc: String,
    pub distro: Distro,
    pub fs: Filesystem,
    pub desktop: Desktop,
    pub rootpasswd: String,
    pub username: String,
    pub passwd: String,
}

impl Install {
    pub fn start(self) {
        self.prepare_emmc();
        self.cgpt_tomfoolery();
        sleep(Duration::from_secs(5));
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

    fn prepare_emmc(&self) {
        let output = Command::new("wipefs")
            .args(["-a", self.emmc.as_str()])
            .output()
            .expect("Failed to wipe eMMC.");
        debug_output(output);
    }
    fn cgpt_tomfoolery(&self) {
        let output = Command::new("dd")
            .args([
                "if=/dev/zero",
                format!("of={}", self.emmc).as_str(),
                "bs=512k",
                "count=128",
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
            .expect("Failed to add kernel partition to eMMC.");
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
                "-s",
                remaining_size
                    .to_string()
                    .as_str(),
                "-l",
                "Root",
                self.emmc.as_str(),
            ])
            .output()
            .expect("Failed to add root partition to eMMC.");
        debug_output(output);

        let output = Command::new("partprobe")
            .output()
            .expect("Failed to run partprobe");
        debug_output(output);
    }

    fn setup_archlinux(&self) {
        create_dir_all("tmp").expect("Failed to create temporary directory.");
        let output = Command::new("mount")
            .args(["-t", "tmpfs", "zinc_tmp", "tmp"])
            .output()
            .expect("Failed to mount temporary folder to RAM.");
        debug_output(output);

        let rootfs_tar = "http://os.archlinuxarm.org/os/ArchLinuxARM-aarch64-latest.tar.gz";

        let client = Client::new();

        let mut response: reqwest::blocking::Response = client
            .get(rootfs_tar)
            .send()
            .expect("Failed to query archlinuxarm.org");

        if response.status().is_success() {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open("tmp/arch.tar.gz")
                .expect("Failed to create tarball tempfile.");
            response
                .copy_to(&mut file)
                .expect("Failed to download archlinuxarm tarball.");
        }

        let output = Command::new("tar")
            .args(["xfp", "tmp/arch.tar.gz", "-C", "/mnt"])
            .output()
            .expect("Failed to extract rootfs tarball into /mnt");
        debug_output(output);

        sleep(Duration::from_secs(5));

        let output = Command::new("umount")
            .arg("tmp")
            .output()
            .expect("Failed to unmount temporary directory");
        debug_output(output);
        remove_dir_all("tmp").expect("Failed to remove temporary directory.");

        let output = Command::new("arch-chroot")
            .args(["/mnt", "/usr/bin/pacman-key", "--init"])
            .output()
            .expect("Failed to run pacman-key --init.");
        debug_output(output);

        let output = Command::new("arch-chroot")
            .args(["/mnt", "/usr/bin/pacman-key", "--populate", "archlinuxarm"])
            .output()
            .expect("Failed to populate archlinuxarm keyring");
        debug_output(output);
        let mut resolv_in = OpenOptions::new()
            .read(true)
            .write(false)
            .open("/etc/resolv.conf")
            .expect("Failed to access /etc/resolv.conf");
        let mut resolv_out = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open("/mnt/etc/resolv.conf")
            .expect("Failed to access/create /mnt/etc/resolv.conf");
        copy(&mut resolv_in, &mut resolv_out)
            .expect("Failed to copy /etc/resolv.conf to /mnt/etc/resolv.conf");

        let output = Command::new("arch-chroot")
            .args([
                "/mnt",
                "/usr/bin/pacman",
                "-Syu",
                "--noconfirm",
                "networkmanager",
            ])
            .output()
            .expect("Failed to install networkmanager to /mnt");
        debug_output(output);

        let output = Command::new("chroot")
            .args([
                "/mnt",
                "systemctl",
                "enable",
                "--now",
                "NetworkManager.service",
            ])
            .output()
            .expect("Failed to start and enable NetworkManager.service on /mnt");
        debug_output(output);

        let output = Command::new("sed")
            .args([
                "-i",
                r"'s/\#en_US.UTF-8/en_US.UTF-8/'",
                "/mnt/etc/locale.gen",
            ])
            .output()
            .expect("Failed to edit /mnt/etc/locale.gen");
        debug_output(output);

        let mut localeconf = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("/mnt/etc/locale.conf")
            .expect("Failed to access/create /mnt/etc/locale.conf");
        writeln!(&mut localeconf, "LANG=en_US.UTF-8")
            .expect("Failed to write into /mnt/etc/locale.conf");

        let output = Command::new("chroot")
            .args(["/mnt", "locale-gen"])
            .output()
            .expect("Failed to generate locales on /mnt");
        debug_output(output);
    }

    fn setup_debian(&self) {
        let output = Command::new("debootstrap")
            .args([
                "--arch=arm64",
                "bookworm",
                "/mnt",
                "https://deb.debian.org/debian/",
            ])
            .output()
            .expect("Failed to run debootstrap.");

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
            Distro::ArchLinux => {
                let output = Command::new("chroot")
                    .args(["/mnt", "/sbin/useradd", "-m", self.username.trim()])
                    .output()
                    .expect("Failed to create user in chroot.");
                debug_output(output);

                let output = Command::new("bash")
                    .args([
                        "-c",
                        format!(r#"echo -e "{}" | passwd {}"#, self.passwd, self.username).as_str(),
                    ])
                    .output()
                    .expect("Failed to set user password.");
                debug_output(output);

                let output = Command::new("bash")
                    .args([
                        "-c",
                        format!(r#"echo -e "{}" | passwd"#, self.rootpasswd).as_str(),
                    ])
                    .output()
                    .expect("Failed to set root password.");
                debug_output(output);
            }
            Distro::Debian => {
                let output = Command::new("chroot")
                    .args(["/mnt", "/sbin/useradd", "-m", self.username.trim()])
                    .output()
                    .expect("Failed to create user in chroot.");
                debug_output(output);
                // need to input password
                let mut child = Command::new("chroot")
                    .args(["/mnt", "passwd", self.username.trim()])
                    .stdin(Stdio::piped())
                    .spawn()
                    .expect("Failed to set user password.");
                let mut stdin = child.stdin.take().expect("Failed to open stdin");
                std::thread::spawn(move || {
                    stdin
                        .write_all(self.passwd.as_bytes())
                        .expect("Failed to write passwd to stdin");
                });
                // need to input root password
                let mut child = Command::new("chroot")
                    .args(["/mnt", "passwd"])
                    .stdin(Stdio::piped())
                    .spawn()
                    .expect("Failed to set root password.");
                let mut stdin = child.stdin.take().expect("Failed to open stdin");
                std::thread::spawn(move || {
                    stdin
                        .write_all(self.rootpasswd.as_bytes())
                        .expect("Failed to write rootpasswd to stdin");
                });
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

pub fn debug_output(output: Output) {
    debug!(
        "status: {}\nstdout: {}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
