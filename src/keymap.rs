//! global keymap logic.
use crate::prelude::*;
use crate::tetromino::has_collision;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameEvent {
  PlayPause,
  ToggleMute,
  NewGame,
  Quit,
}

pub(crate) fn handle_game_events(
  mut events: EventReader<GameEvent>,
  mut app_state: ResMut<NextState<AppState>>,
  mut game_state: ResMut<NextState<GameState>>,
  mut exit: EventWriter<AppExit>,
) {
  for &evt in events.iter() {
    if app_state.0 == Some(AppState::InGame) {
      if game_state.0 == Some(GameState::Playing) {
        game_state.set(GameState::Paused);
      } else if game_state.0 == Some(GameState::Paused) {
        game_state.set(GameState::Playing);
      }
    } else if evt == GameEvent::NewGame {
      app_state.set(AppState::InGame);
      game_state.set(GameState::Restarting);
    } else if evt == GameEvent::Quit {
      exit.send(AppExit);
    }
  }
}

pub(crate) fn emit_game_events(
  kb: Res<Input<KeyCode>>,
  mut events: EventWriter<GameEvent>,
) {
  let keymap = if kb.just_pressed(KeyCode::Space) {
    Some(GameEvent::PlayPause)
  } else if kb.just_pressed(KeyCode::N) {
    Some(GameEvent::NewGame)
  } else if kb.just_pressed(KeyCode::M) {
    Some(GameEvent::ToggleMute)
  } else {
    None
  };
  // if the above keymap is not None, send it to the event channel for subscribers downstream.
  if let Some(keymap) = keymap {
    info!("{:?}", keymap);
    events.send(keymap);
  }
}

/// self-contained [`System`] that handles the quit keymap for all operating systems.
#[allow(unused)]
pub fn handle_quit(kb: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
  let ctrl = kb.any_pressed([KeyCode::LControl, KeyCode::RControl]);
  let alt = kb.any_pressed([KeyCode::LAlt, KeyCode::RAlt]);
  let win = kb.any_pressed([KeyCode::LWin, KeyCode::RWin]);
  if (ctrl && kb.just_pressed(KeyCode::W))
    || (alt && kb.just_pressed(KeyCode::F4))
    || (win && kb.just_pressed(KeyCode::W))
  {
    exit.send(AppExit);
  }
}

#[allow(unused)]
pub(crate) fn hold_piece(
  mut commands: Commands,
  kb: Res<Input<KeyCode>>,
  mut q: Query<(Entity, &Movable, &Shape), With<Shape>>,
  mut data: ResMut<GameBoardData>,
) {
  // bail if player already held a piece this round, or if not pressing the hold key.
  if data.held || !kb.just_pressed(KeyCode::H) {
    return;
  }
  q.iter().next().map(|(e, _movable, shape)| {
    data.hold(shape.to_owned());
  });
  // TODO: Despawn the falling piece, so that a new one can be spawned.
}

pub(crate) fn move_piece(
  mut query: Query<(&mut Tile, &mut Transform, &Movable), With<Shape>>,
  kb: Res<Input<KeyCode>>,
  mut timers: ResMut<GameTimers>,
  t: Res<Time>,
  audio: Res<Audio>,
  score: Res<Score>,
  game_audio: Res<GameAssets>,
) {
  timers.manual.tick(t.delta());
  timers.auto.tick(
    t.delta()
      .add(GameTimers::calculate_score_speed_offset(score)),
  );

  // do not allow the piece to move downwards 2 tiles at a time.
  let mut already_down = false;

  // if the auto timer just finished, move the piece down.
  if timers.auto.just_finished() {
    for (mut block, mut transform, mv) in &mut query {
      if mv.can_down {
        block.1 -= 1;
        already_down = true;
        game_audio.play_drop(&audio);
      }
      transform.translation = block.translation();
    }
  }

  if timers.manual.finished() {
    let move_left = kb.any_pressed([KeyCode::A, KeyCode::Left]);
    let move_right = kb.any_pressed([KeyCode::D, KeyCode::Right]);
    let move_down = kb.any_pressed([KeyCode::S, KeyCode::Down]);
    let rotate = kb.any_just_pressed([KeyCode::Up, KeyCode::W]);
    for (mut block, mut transform, mv) in &mut query {
      if move_left && mv.can_left {
        block.0 -= 1;
        timers.manual.reset();
        transform.translation = block.translation();
      } else if move_right && mv.can_right {
        block.0 += 1;
        timers.manual.reset();
        transform.translation = block.translation();
      } else if move_down && mv.can_down && !already_down {
        block.1 -= 1;
        timers.manual.reset();
        transform.translation = block.translation();
      } else if rotate {
        // not implemented (here) yet. see rotate_piece() instead.
        return;
      }
    }
  }
}

pub(crate) fn rotate_piece(
  kb: Res<Input<KeyCode>>,
  mut q_piece: Query<(&mut Shape, &mut Tile, &mut Transform)>,
  q_board: Query<&Tile, Without<Shape>>,
) {
  if !kb.any_just_pressed([KeyCode::Up, KeyCode::W]) {
    return;
  }
  let piece_type = match q_piece.iter().next() {
    Some((piece_type, _, _)) => *piece_type,
    None => {
      return;
    }
  };
  let sum_x = q_piece.iter().map(|(_, block, _)| block.0).sum::<i32>();
  let sum_y = q_piece.iter().map(|(_, block, _)| block.1).sum::<i32>();

  let original_blocks: Vec<Tile> = q_piece.iter().map(|(_, block, _)| *block).collect();
  for (_, mut block, mut transform) in &mut q_piece {
    *block = match piece_type {
      Shape::O | Shape::L | Shape::J => Tile::shift(
        [block.1, -block.0].into(),
        Some(sum_x / 4 - sum_y / 4),
        Some(sum_x / 4 + sum_y / 4 + 1),
      ),
      _ => Tile::shift(
        [block.1, -block.0].into(),
        Some(sum_x / 4 - sum_y / 4),
        Some(sum_x / 4 + sum_y / 4),
      ),
    };
    transform.translation = block.translation();
  }

  if has_collision(&q_piece, &q_board) {
    for (_, mut block, mut transform) in &mut q_piece {
      *block = Tile::shift(*block, Some(-1), None);
      transform.translation = block.translation();
    }
  }

  if has_collision(&q_piece, &q_board) {
    for (_, mut block, mut transform) in &mut q_piece {
      *block = Tile::shift(*block, Some(-1), None);
      transform.translation = block.translation();
    }
  }

  if has_collision(&q_piece, &q_board) {
    for (_, mut block, mut transform) in &mut q_piece {
      *block = Tile::shift(*block, Some(3), None);
      transform.translation = block.translation();
    }
  }

  if has_collision(&q_piece, &q_board) {
    for (_, mut block, mut transform) in &mut q_piece {
      *block = Tile::shift(*block, Some(3), None);
      transform.translation = block.translation();
    }
  }

  if has_collision(&q_piece, &q_board) {
    let mut index = 0;
    for (_, mut block, mut transform) in &mut q_piece {
      *block = original_blocks[index];
      transform.translation = block.translation();
      index += 1;
    }
  }
}
