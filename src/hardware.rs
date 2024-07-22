use std::fs::{self, read_dir};

use crate::BOARDS;

#[derive(Clone, Copy)]
pub enum Baseboard {
    Gru,
    Kukui, 
    Oak,
    Trogdor,
    Veyron,
    None
}

impl From<&&str> for Baseboard {
    fn from(value: &&str) -> Self {
        match value {
            &"gru" => Self::Gru,
            &"kukui" => Self::Kukui,
            &"oak" => Self::Oak,
            &"trogdor" => Self::Trogdor,
            &"veyron" => Self::Veyron,
            _ => Self::None
        }
    }
}

impl From<Board> for Baseboard {
    fn from(value: Board) -> Self {
        match value {
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
        }
    }
}

#[derive(Clone, Copy)]
pub enum Board {
    Bob,
    Coachz,
    Hana,
    Homestar,
    Kevin,
    Kodama,
    Krane,
    Lazor,
    Minnie,
    Speedy,
    None
}

impl Board {
    pub fn get() -> Self {
        let hardware_raw = fs::read_to_string("/sys/firmware/devicetree/base/compatible").expect("Failed to get board info.");

        BOARDS.iter().find(|b| hardware_raw.contains(*b)).expect("Your board isnt supported. (How did you boot this?)").into()
    }
}

impl From<&&str> for Board {
    fn from(value: &&str) -> Self {
        match value {
            &"bob" => Self::Bob,
            &"coachz" => Self::Coachz,
            &"hana" => Self::Hana,
            &"homestar" => Self::Homestar,
            &"kevin" => Self::Kevin,
            &"kodama" => Self::Kodama,
            &"krane" => Self::Krane,
            &"lazor" => Self::Lazor,
            &"minnie" => Self::Minnie,
            &"speedy" => Self::Speedy,
            _ => Self::None
        }
    }
}

pub fn get_emmc() -> Option<String> {
    let dev = read_dir("/dev").expect("Failed to list /dev.");
    for path in dev {
        if let Ok(path) = path {
            eprintln!("{}", path.path().to_string_lossy().to_string());
            if path.path().to_string_lossy().to_string() != "/dev/mmcblk0"
            || path.path().to_string_lossy().to_string() != "/dev/mmcblk1" {
                continue;
            }
            return Some(path.path().to_string_lossy().to_string());
        }
    }
    None
}