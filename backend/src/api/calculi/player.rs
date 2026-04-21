pub type PlayerID = i8;

pub struct Player {
    id: PlayerID
}

impl Player {
    pub fn new(id: PlayerID) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> PlayerID {
        self.id
    }
}