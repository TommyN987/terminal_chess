use uuid::Uuid;

pub struct GameId(Uuid);

impl From<Uuid> for GameId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Debug)]
pub struct PlayerId(Uuid);

impl From<Uuid> for PlayerId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}
