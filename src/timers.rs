//! timers and their associated logic.
//!
//! - manual movements. (left, right, down)
//! - auto movements. (falling on interval)
//! - rm_pieces. (removing pieces from the board after clear)
use crate::prelude::*;

/// resource that keeps track of all game-related timers for moving the piece
/// down automatically as well as manually (by the player).
#[derive(Debug, Resource, Reflect)]
pub struct GameTimers {
  /// timer between manually input moves by the player.
  pub manual: Timer,
  /// timer between automatic moves of the falling piece.
  pub auto: Timer,
  /// timer between removing pieces from the board.
  pub rm_pieces: Timer,
}

impl FromWorld for GameTimers {
  fn from_world(_world: &mut World) -> Self {
    let mut manual = Timer::new(std::time::Duration::from_millis(100), TimerMode::Once);
    manual.set_elapsed(std::time::Duration::from_millis(100));
    let mut rm_pieces = Timer::new(std::time::Duration::from_millis(300), TimerMode::Once);
    rm_pieces.set_elapsed(std::time::Duration::from_millis(300));
    let mut auto = Timer::new(std::time::Duration::from_millis(1000), TimerMode::Repeating);
    auto.set_elapsed(std::time::Duration::from_millis(1000));
    Self {
      manual,
      rm_pieces,
      auto,
    }
  }
}

impl GameTimers {
  /// add a small offset to the automove timer to make the game progress faster as player progresses.
  /// TODO: fix this scaling hack. this should scare parabolically, asymptotic to the player's score.
  #[must_use]
  pub fn calculate_score_speed_offset(score: Res<Score>) -> Duration {
    let level = score.points / 100;
    Duration::from_millis(u64::from(level))
  }
}
