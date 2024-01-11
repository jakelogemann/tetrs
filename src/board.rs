//! the game board and its logic.
use crate::prelude::*;

/// the overall game board.
pub struct GameBoard;

impl Plugin for GameBoard {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GameBoardData::default())
      .add_startup_system(Self::setup)
      .add_system(Tile::clear_all.in_schedule(OnEnter(GameState::Restarting)));
  }
}

#[derive(Resource, Debug, Default, Copy, Clone)]
pub struct GameBoardData {
  /// has the player held a piece (since the last piece was spawned)?
  pub held: bool,
  /// the piece that the player is holding.
  pub held_piece: Option<Shape>,
  /// next piece to be spawned.
  pub next_piece: Option<Shape>,
}

impl GameBoardData {
  pub fn hold(&mut self, piece: Shape) {
    self.held = true;
    self.held_piece = Some(piece);
  }
}

impl GameBoard {
  /// how many block units wide is the board?
  pub const COL_COUNT: u8 = 10;
  /// how many block units tall is the board?
  pub const ROW_COUNT: u8 = 20;
  /// how many total spaces is the board?
  #[allow(unused)]
  pub const SPACES: u8 = Self::ROW_COUNT * Self::COL_COUNT;
  /// how thick is the border of the game?
  pub const BORDER_THICKNESS: f32 = 10.0;
  // what color is the border of the board?
  pub const BORDER_COLOR: Color = palette::SURFACE1;

  /// setup the game board [system][`System`].
  fn setup(mut commands: Commands) {
    let col_midpoint = f32::from(GameBoard::COL_COUNT) / 2.0;
    let row_midpoint = f32::from(GameBoard::ROW_COUNT) / 2.0;
    let z = 0.0;
    let wall_sprite = Sprite {
      color: GameBoard::BORDER_COLOR,
      ..default()
    };
    commands
      .spawn(SpriteBundle {
        transform: Transform {
          translation: Vec3 {
            x: (-col_midpoint).mul_add(Tile::LENGTH, -GameBoard::BORDER_THICKNESS / 2.0),
            ..default()
          },
          scale: Vec3 {
            x: GameBoard::BORDER_THICKNESS,
            y: f32::from(GameBoard::ROW_COUNT)
              .mul_add(Tile::LENGTH, 2.0 * GameBoard::BORDER_THICKNESS),
            z,
          },
          ..default()
        },
        sprite: wall_sprite.clone(),
        ..default()
      })
      .insert(Name::new("GameBoard Wall (Left)"));
    commands
      .spawn(SpriteBundle {
        transform: Transform {
          translation: Vec3 {
            x: col_midpoint.mul_add(Tile::LENGTH, GameBoard::BORDER_THICKNESS / 2.0),
            ..default()
          },
          scale: Vec3 {
            x: GameBoard::BORDER_THICKNESS,
            y: f32::from(GameBoard::ROW_COUNT)
              .mul_add(Tile::LENGTH, 2.0 * GameBoard::BORDER_THICKNESS),
            z,
          },
          ..default()
        },
        sprite: wall_sprite.clone(),
        ..default()
      })
      .insert(Name::new("GameBoard Wall (Right)"));
    commands
      .spawn(SpriteBundle {
        transform: Transform {
          translation: Vec3 {
            y: row_midpoint.mul_add(Tile::LENGTH, GameBoard::BORDER_THICKNESS / 2.0),
            ..default()
          },
          scale: Vec3 {
            x: f32::from(GameBoard::COL_COUNT) * Tile::LENGTH,
            y: GameBoard::BORDER_THICKNESS,
            z,
          },
          ..default()
        },
        sprite: wall_sprite.clone(),
        ..default()
      })
      .insert(Name::new("GameBoard Wall (Top)"));
    commands
      .spawn(SpriteBundle {
        transform: Transform {
          translation: Vec3 {
            y: (-row_midpoint).mul_add(Tile::LENGTH, -GameBoard::BORDER_THICKNESS / 2.0),
            ..default()
          },
          scale: Vec3 {
            x: f32::from(GameBoard::COL_COUNT) * Tile::LENGTH,
            y: GameBoard::BORDER_THICKNESS,
            z,
          },
          ..default()
        },
        sprite: wall_sprite,
        ..default()
      })
      .insert(Name::new("GameBoard Wall (Bottom)"));
  }


/// a [`System`] which controls the visibility of the [`Tile`]s. if they are
/// above the top of the board, they are hidden.
pub fn update_tile_visibility(mut q: Query<(&mut Visibility, &Tile), With<Shape>>) {
  for (mut vis, block) in &mut q {
    *vis = if block.1 > (GameBoard::ROW_COUNT as i32 - 1) {
      Visibility::Hidden
    } else {
      Visibility::Visible
    };
  }
}
}

/// [system][`System`] responsible for detecting a "game over" state and transitioning to it.
pub fn detect_game_over(
  mut app_state: ResMut<NextState<AppState>>,
  mut game_state: ResMut<NextState<GameState>>,
  query: Query<&Tile, Without<Shape>>,
  audio: Res<Audio>,
  game_audio: Res<GameAssets>,
) {
  let mut max_block_y = 0;
  for block in &query {
    if block.1 > max_block_y {
      max_block_y = block.1;
    }
  }
  // info!("max_block_y: {}", max_block_y);
  if max_block_y >= (GameBoard::ROW_COUNT as i32 - 1) {
    game_audio.play_gameover(&audio);
    app_state.set(AppState::GameOver);
    game_state.set(GameState::Quitting);
  }
}

/// [system][`System`] responsible for unmarking falling pieces as [`Movable`].
pub fn remove_piece_component(
  mut commands: Commands,
  q_piece_blocks: Query<(Entity, &Movable), With<Shape>>,
  mut game_timers: ResMut<GameTimers>,
  kb: Res<Input<KeyCode>>,
  time: Res<Time>,
) {
  if !q_piece_blocks.is_empty() && !q_piece_blocks.iter().last().unwrap().1.can_down {
    if !q_piece_blocks.iter().last().unwrap().1.can_down {
      game_timers.rm_pieces.tick(time.delta());
    } else {
      game_timers.rm_pieces.reset();
    }
  }
  let mut reset_timer = false;
  for (entity, movable) in &q_piece_blocks {
    if !movable.can_down && (game_timers.rm_pieces.just_finished() || kb.pressed(KeyCode::Down)) {
      commands.entity(entity).remove::<Shape>();
      reset_timer = true;
    }
  }
  if reset_timer {
    game_timers.rm_pieces.reset();
  }
}

pub fn check_full_line(
  mut commands: Commands,
  mut score: ResMut<Score>,
  mut query: Query<(Entity, &mut Tile, &mut Transform), Without<Shape>>,
  audio: Res<Audio>,
  game_audio: Res<GameAssets>,
) {
  let mut y_to_x_set_map: HashMap<i32, HashSet<i32>> = HashMap::new();
  for (_, block, _) in &query {
    if let std::collections::hash_map::Entry::Vacant(e) = y_to_x_set_map.entry(block.1) {
      let mut x_set = HashSet::new();
      x_set.insert(block.0);
      e.insert(x_set);
    } else {
      let x_set = y_to_x_set_map.get_mut(&block.1).unwrap();
      x_set.insert(block.0);
    }
  }

  // identify rows which are completely filled with blocks.
  let mut full_rows = Vec::new();
  for (y, x_set) in &y_to_x_set_map {
    if x_set.len() == GameBoard::COL_COUNT as usize {
      full_rows.push(y);
    }
  }

  // if no lines to clear, we're done.
  if full_rows.is_empty() {
    return;
  }

  // otherwise, play audio and update score.
  game_audio.play_line_clear(&audio);
  score.update(full_rows.len());

  // despawn the blocks in the full lines.
  let mut despawn_entities = Vec::new();
  for line_no in &full_rows {
    let line_no = line_no.clone().to_owned();
    for (entity, block, _) in &mut query {
      if block.1 == line_no {
        despawn_entities.push(entity);
        commands.entity(entity).despawn();
      }
    }
  }

  // move down the blocks above the full lines.
  full_rows.sort();
  full_rows.reverse();
  for line_no in &full_rows {
    for (entity, mut block, mut transform) in &mut query {
      if !despawn_entities.contains(&entity) && block.1 > *line_no.clone() {
        debug!(
          "block ({},{}) fell (line_no={}).",
          block.0, block.1, line_no
        );
        block.1 -= 1;
        transform.translation = block.translation();
      }
    }
  }
}
