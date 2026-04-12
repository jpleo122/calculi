use owo_colors::{Style};

pub type PlayerID = i8;

pub struct Player {
    id: PlayerID,
    style: Style
}

impl Player {
    pub fn new(id: PlayerID, style: Style) -> Self {
        Self { id, style }
    }

    pub fn get_id(&self) -> PlayerID {
        self.id
    }

    pub fn get_style(&self) -> Style {
        self.style
    }
}