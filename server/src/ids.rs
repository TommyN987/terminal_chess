use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GameId(Uuid);

impl From<Uuid> for GameId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerId(Uuid);

impl From<Uuid> for PlayerId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}
