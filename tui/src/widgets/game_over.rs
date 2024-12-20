use domain::{game::GameResult, game::Player};

pub struct GameOver {
    pub winner: Option<Player>,
}

impl From<GameResult> for GameOver {
    fn from(value: GameResult) -> Self {
        Self {
            winner: value.winner,
        }
    }
}
