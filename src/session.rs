use games::tic_tac_toe::{self, CellIndex};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConnectFourSession {
    id: Uuid,
    player1_id: Option<Uuid>,
    player2_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicTacToeUpdate {
    pub session_id: Uuid,
    pub player_id: Uuid,
    pub cell_index: CellIndex,
}

#[derive(Clone, Copy, Serialize, Debug, Deserialize)]
pub struct TicTacToeSession {
    pub state: tic_tac_toe::GameState,
    player1_id: Option<Uuid>,
    player2_id: Option<Uuid>,
}
impl Default for TicTacToeSession {
    fn default() -> Self {
        return Self {
            state: tic_tac_toe::GameState::default(),
            player1_id: None,
            player2_id: None,
        };
    }
}
impl TicTacToeSession {
    pub fn player1_id(&self) -> Option<Uuid> {
        return self.player1_id;
    }
    pub fn player2_id(&self) -> Option<Uuid> {
        return self.player2_id;
    }
    pub fn capacity(&self) -> SessionCapacity {
        match [self.player1_id, self.player2_id] {
            [None, None] => SessionCapacity::Empty,
            [Some(_), None] => SessionCapacity::OnlyPlayer1,
            [None, Some(_)] => SessionCapacity::OnlyPlayer2,
            [Some(_), Some(_)] => SessionCapacity::Full,
        }
    }
    /// This function adds player1 to the session if there is no player 1. The player id is returned if a player was added
    pub fn add_player1(&mut self) {
        if self.player1_id.is_none() {
            self.player1_id = Some(Uuid::new_v4());
        }
    }
    /// This function adds player2 to the session if there the spot is not yet occupied.
    pub fn add_player2(&mut self) {
        if self.player2_id.is_none() {
            self.player2_id = Some(Uuid::new_v4());
        }
    }
    pub fn remove_player1(&mut self) {
        if self.player1_id.is_some() {
            self.player1_id = None;
        }
    }
    pub fn remove_player2(&mut self) {
        if self.player2_id.is_some() {
            self.player2_id = None;
        }
    }
}
pub enum SessionCapacity {
    Empty,
    OnlyPlayer1,
    OnlyPlayer2,
    Full,
}
impl SessionCapacity {
    pub fn is_full(&self) -> bool {
        return if let SessionCapacity::Full = self {
            true
        } else {
            false
        };
    }
    pub fn is_empty(&self) -> bool {
        return if let SessionCapacity::Empty = self {
            true
        } else {
            false
        };
    }
}
