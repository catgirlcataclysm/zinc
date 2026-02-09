use std::fs::{self, read_dir};

pub fn get_emmc() -> Option<String> {
    let dev = read_dir("/dev").expect("Failed to list /dev.");
    let path = dev.into_iter().find_map(|p| {
        let path = p.unwrap().path().to_string_lossy().trim().to_string();
        //TODO: fix this pattern matching to match any mmcblk device and/or then let user choose where to install 
        if &path == "/dev/mmcblk0" || &path == "/dev/mmcblk1" {
            Some(path)
        } else {
            None
        }
    });
    path
}
