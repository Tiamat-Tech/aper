use aper::{
    data_structures::{atom::Atom, fixed_array::FixedArray},
    Aper, AperSync, IntentMetadata, Store,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(AperSync, Clone)]
struct TicTacToe {
    grid: FixedArray<9, Option<TicTacToePlayer>>,
    player: Atom<TicTacToePlayer>,
    winner: Atom<Option<TicTacToePlayer>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq)]
enum TicTacToePlay {
    Play(u8),
    Reset,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug, Default)]
enum TicTacToePlayer {
    #[default]
    X,
    O,
}

fn check_winner(grid: Vec<Option<TicTacToePlayer>>) -> Option<TicTacToePlayer> {
    let winning_combinations = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],
        vec![0, 3, 6],
        vec![1, 4, 7],
        vec![2, 5, 8],
        vec![0, 4, 8],
        vec![2, 4, 6],
    ];

    for combination in winning_combinations {
        let player = grid[combination[0]];

        if player.is_none() {
            continue;
        }

        if grid[combination[1]] == player && grid[combination[2]] == player {
            return player;
        }
    }

    None
}

impl Aper for TicTacToe {
    type Intent = TicTacToePlay;
    type Error = ();

    fn apply(
        &mut self,
        intent: &Self::Intent,
        _metadata: &IntentMetadata,
    ) -> Result<(), Self::Error> {
        let player = self.player.get();

        match &intent {
            TicTacToePlay::Play(cell) => {
                self.grid.set(*cell as u32, Some(player));
                self.player.set(match player {
                    TicTacToePlayer::X => TicTacToePlayer::O,
                    TicTacToePlayer::O => TicTacToePlayer::X,
                });

                // Check for win

                let grid: Vec<Option<TicTacToePlayer>> = self.grid.iter().collect();
                if let Some(winner) = check_winner(grid) {
                    self.winner.set(Some(winner));
                }
            }
            TicTacToePlay::Reset => {
                for i in 0..9 {
                    self.grid.set(i, None);
                }
            }
        }

        Ok(())
    }
}

#[test]
fn test_tic_tac_toe() {
    let map = Store::default();
    let mut game = TicTacToe::attach(map.handle());

    game.apply(
        &TicTacToePlay::Play(0),
        &IntentMetadata::new(None, Utc::now()),
    )
    .unwrap(); // X
    game.apply(
        &TicTacToePlay::Play(1),
        &IntentMetadata::new(None, Utc::now()),
    )
    .unwrap(); // O
    game.apply(
        &TicTacToePlay::Play(3),
        &IntentMetadata::new(None, Utc::now()),
    )
    .unwrap(); // X
    game.apply(
        &TicTacToePlay::Play(2),
        &IntentMetadata::new(None, Utc::now()),
    )
    .unwrap(); // O

    assert_eq!(game.grid.get(0), Some(TicTacToePlayer::X));
    assert_eq!(game.grid.get(1), Some(TicTacToePlayer::O));
    assert_eq!(game.grid.get(3), Some(TicTacToePlayer::X));
    assert_eq!(game.grid.get(2), Some(TicTacToePlayer::O));

    assert_eq!(game.winner.get(), None);

    game.apply(
        &TicTacToePlay::Play(6),
        &IntentMetadata::new(None, Utc::now()),
    )
    .unwrap(); // X for the win
    assert_eq!(game.winner.get(), Some(TicTacToePlayer::X));

    game.apply(&TicTacToePlay::Reset, &IntentMetadata::now())
        .unwrap();

    for i in 0..9 {
        assert_eq!(game.grid.get(i), None);
    }
}
