#[derive(Clone, Copy)]
pub enum Baseboard {
    Gru,
    Kukui, 
    Oak,
    Trogdor,
    Veyron,
    None
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