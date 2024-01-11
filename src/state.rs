//! state of the application/game.
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default, Reflect)]
/// game's current state ("playing", "paused", etc).
pub enum GameState {
  /// GamePlaying is the state when the game is currently playing.
  Playing,
  /// GamePaused is the state when the game is currently paused.
  Paused,
  Restarting,
  #[default]
  Quitting,
}

impl GameState {
  #[must_use]
  pub fn is_playing_state(s: Res<State<Self>>) -> bool {
    s.0 == Self::Playing
  }
  /// returns true if the current state is [`Self::Playing`].
  #[must_use]
  pub fn is_playing(&self) -> bool {
    matches!(self, Self::Playing)
  }
  /// returns true if the current state is [`Self::Quitting`].
  #[must_use]
  pub fn is_quitting(&self) -> bool {
    matches!(self, Self::Quitting)
  }
  /// returns true if the current state is [`Self::Restarting`].
  #[must_use]
  pub fn is_restarting(&self) -> bool {
    matches!(self, Self::Restarting)
  }
  /// returns true if the current state is [`Self::Paused`].
  #[must_use]
  pub fn is_paused(&self) -> bool {
    matches!(self, Self::Paused)
  }
  /// returns true if the current state is considered "in a game".
  #[must_use]
  pub fn in_a_game(&self) -> bool {
    self.is_playing() || self.is_paused()
  }
}

/// overall application state, distinct from the state of a game.
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default, Reflect)]
pub enum AppState {
  /// MainMenu is the state when the game is currently in the main menu. This is the default state.
  #[default]
  MainMenu,
  /// InGame is the state when the game is currently playing.
  InGame,
  /// GameOver is the state when the game has finished and the player may
  /// restart or quit.
  GameOver,
}

impl AppState {
  /// returns true if the current state is [`Self::MainMenu`].
  #[must_use]
  pub fn in_menu(&self) -> bool {
    matches!(self, Self::MainMenu)
  }
  /// returns true if the current state is [`Self::InGame`].
  #[must_use]
  pub fn in_game(&self) -> bool {
    matches!(self, Self::InGame)
  }
  /// returns true if the current state is [`Self::GameOver`].
  #[must_use]
  pub fn game_over(&self) -> bool {
    matches!(self, Self::GameOver)
  }
}

pub fn pause_game(
  game_state: Res<State<GameState>>,
  mut change_game_state: ResMut<NextState<GameState>>,
  keyboard_input: Res<Input<KeyCode>>,
) {
  if keyboard_input.just_pressed(KeyCode::Escape) {
    if game_state.0 == GameState::Playing {
      change_game_state.set(GameState::Paused);
    } else {
      change_game_state.set(GameState::Playing);
    }
  }
}

pub fn play_game(mut game_state: ResMut<NextState<GameState>>) {
  game_state.set(GameState::Playing);
}
