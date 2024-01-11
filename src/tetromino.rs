//! [`Tetromino`]'s logic, rotations, etc.
use crate::prelude::*;

/// represents all the logical types of pieces within the game.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Reflect)]
pub enum Shape {
  // ####
  /// The I piece is 4 blocks in a line.
  ///
  /// ```text
  ///  ####
  /// ```
  I,

  /// The J piece is 4 blocks in a line with a hook.
  ///
  /// ```text
  /// #
  /// ###
  /// ```
  J,

  /// The L piece is 4 blocks in a line with a hook, opposite of a J.
  ///
  /// ```text
  ///   #
  /// ###
  /// ```
  L,

  /// The O piece is 4 block arranged in a 2x2 square. Rotation has no effect.
  ///
  /// ```text
  /// ##
  /// ##
  /// ```
  O,

  /// The S piece is a 2x2 square with the top two blocks horizonatlly offset by +1.
  /// ```text
  ///  ##
  /// ##
  /// ```
  S,

  /// The T piece is a 3 blocks in a line with a 4th block below the middle.
  ///
  /// ```text
  ///  #
  /// ###
  /// ```
  T,

  /// The Z piece is a 2x2 square with the top two blocks horizonatlly offset by -1.
  /// ```text
  /// ##
  ///  ##
  /// ```
  // ##
  //  ##
  Z,
}

impl fmt::Display for Shape {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", &self)
  }
}

impl Shape {
  pub const VARIANTS: u32 = 7;

  const SHAPE_I: [[i32; 2]; 4] = [[3, 0], [4, 0], [5, 0], [6, 0]];
  const SHAPE_J: [[i32; 2]; 4] = [[3, 1], [3, 0], [4, 0], [5, 0]];
  const SHAPE_L: [[i32; 2]; 4] = [[3, 0], [4, 0], [5, 0], [5, 1]];
  const SHAPE_O: [[i32; 2]; 4] = [[4, 1], [4, 0], [5, 1], [5, 0]];
  const SHAPE_S: [[i32; 2]; 4] = [[3, 0], [4, 0], [4, 1], [5, 1]];
  const SHAPE_T: [[i32; 2]; 4] = [[3, 0], [4, 1], [4, 0], [5, 0]];
  const SHAPE_Z: [[i32; 2]; 4] = [[3, 1], [4, 1], [4, 0], [5, 0]];

  #[must_use]
  pub fn shape(&self) -> [Tile; 4] {
    Self::shape_of(self)
  }

  pub fn shape_of(other: &Self) -> [Tile; 4] {
    match other {
      Self::I => Self::SHAPE_I,
      Self::J => Self::SHAPE_J,
      Self::L => Self::SHAPE_L,
      Self::O => Self::SHAPE_O,
      Self::S => Self::SHAPE_S,
      Self::T => Self::SHAPE_T,
      Self::Z => Self::SHAPE_Z,
    }
    .map(std::convert::Into::into)
  }

  /// returns the default color which corresponds to this piece type.
  #[inline(always)]
  #[must_use]
  pub fn color(&self) -> Color {
    use crate::palette::{BLUE, GREEN, MAUVE, PEACH, RED, TEAL, YELLOW};
    match self {
      Self::I => TEAL,
      Self::J => BLUE,
      Self::L => PEACH,
      Self::O => YELLOW,
      Self::S => GREEN,
      Self::T => MAUVE,
      Self::Z => RED,
    }
  }

  /// returns a random shape.
  #[must_use]
  pub fn random() -> Self {
    match rand::thread_rng().gen_range(0..Self::VARIANTS) {
      0 => Self::I,
      1 => Self::J,
      2 => Self::L,
      3 => Self::O,
      4 => Self::S,
      5 => Self::T,
      6 => Self::Z,
      _ => panic!("impossible random piece type"),
    }
  }

  /// returns a set of random shapes .
  #[must_use]
  pub fn random_n(n: usize) -> BTreeSet<Self> {
    let mut s = BTreeSet::new();
    for _ in 0..n {
      s.insert(Self::random());
    }
    s
  }
}

/// represents an actual piece, on the board, in play.
///
/// NOTE: this is necessary to "own" the [`TetrominoBlock`] entities.
#[derive(Debug, Clone, PartialEq)]
pub struct Tetromino {
  /// the shape of this piece.
  pub shape: Shape,
  /// the blocks that make up this piece.
  pub blocks: [Tile; 4],
}

impl Tetromino {
  /// spawn a moveable piece.
  pub fn spawn_movable(self, mut commands: Commands) {
    let uid = rand::thread_rng().gen::<u16>();
    for (i, block) in self.blocks.into_iter().enumerate() {
      let name = Name::new(format!("{}{} (#{})", self.shape.to_string(), i, uid));
      commands
        .spawn(self.shape)
        .insert(block.new_sprite(self.shape.color(), Visibility::Hidden))
        .insert(block)
        .insert(name)
        .insert(Movable {
          can_down: true,
          can_left: true,
          can_right: true,
        });
    }
  }

  /// spawn a unmoveable piece.
  pub fn spawn_frozen(self, mut commands: Commands) {
    let uid = rand::thread_rng().gen::<u16>();
    for (i, block) in self.blocks.into_iter().enumerate() {
      let name = Name::new(format!("{}{} (#{})", self.shape.to_string(), i, uid));
      commands
        .spawn(self.shape)
        .insert(block.new_sprite(self.shape.color(), Visibility::Hidden))
        .insert(block)
        .insert(name);
    }
  }

  #[must_use]
  pub fn random_n(n: usize) -> Vec<Self> {
    // info!("random 7 pieces: {:?}", result);
    Shape::random_n(n)
      .iter()
      .map(|p| Self {
        shape: *p,
        blocks: Self::shift_piece(p.shape(), None, Some(20)),
      })
      .collect()
  }

  #[must_use]
  pub fn shift_piece(
    mut blocks: [Tile; 4],
    delta_x: Option<i32>,
    delta_y: Option<i32>,
  ) -> [Tile; 4] {
    match delta_x {
      Some(delta) => {
        blocks[0].0 += delta;
        blocks[1].0 += delta;
        blocks[2].0 += delta;
        blocks[3].0 += delta;
      }
      None => {}
    }
    match delta_y {
      Some(delta) => {
        blocks[0].1 += delta;
        blocks[1].1 += delta;
        blocks[2].1 += delta;
        blocks[3].1 += delta;
      }
      None => {}
    }
    blocks
  }
}

/// a [`Component`] which represents the ability of an [`Entity`] to move.
#[derive(Component, Reflect)]
pub struct Movable {
  /// can this movable entity move downwards?
  pub can_down: bool,
  /// can this movable entity move left?
  pub can_left: bool,
  /// can this movable entity move right?
  pub can_right: bool,
}

impl Movable {
  /// marks blocks of selected piece(s) which are colliding (touching) the board
  /// or another block. This is used to determine whether the piece can be moved
  /// or rotated.
  pub fn update_pieces(
    mut piece_query: Query<(&mut Tile, &mut Movable), With<Shape>>,
    board_query: Query<&Tile, Without<Shape>>,
  ) {
    let mut can_down = true;
    let mut can_left = true;
    let mut can_right = true;

    for (block, _) in &mut piece_query {
      if block.0 == 0 {
        can_left = false;
      }
      if block.0 == 9 {
        can_right = false;
      }
      if block.1 == 0 {
        can_down = false;
      }
    }

    for (block, _) in &piece_query {
      for board_block in &board_query {
        if board_block.1 == block.1 && block.0 > 0 && board_block.0 == block.0 - 1 {
          can_left = false;
        }
        if board_block.1 == block.1 && board_block.0 == block.0 + 1 {
          can_right = false;
        }
        if board_block.0 == block.0 && block.1 > 0 && board_block.1 == block.1 - 1 {
          can_down = false;
        }
      }
    }

    // update the MoveablePiece component with our new values.
    for (_, mut movable) in &mut piece_query {
      movable.can_left = can_left;
      movable.can_right = can_right;
      movable.can_down = can_down;
    }
  }
}

/// represents the queue of incoming game pieces.
#[derive(Debug, Resource)]
pub struct Queue(pub VecDeque<Tetromino>);

/// a tile represents a single (fixed size) part of a [`Tetromino`]. they are
/// made up of multiple of these blocks.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub struct Tile(pub i32, pub i32);

impl Tile {
  /// the length of a block.
  pub const LENGTH: f32 = 30.0;
  /// the length of a sticker.
  pub const STICKER_LENGTH: f32 = 28.0;

  /// [system][`System`] which clears all the blocks from the board.
  pub fn clear_all(mut commands: Commands, query: Query<Entity, With<Self>>) {
    for entity in &query {
      commands.entity(entity).despawn();
    }
  }

  #[must_use]
  pub fn translation(&self) -> Vec3 {
    Vec3 {
      x: (self.0 as f32 - (f32::from(super::GameBoard::COL_COUNT) / 2.0) + 0.5) * Self::LENGTH,
      y: (self.1 as f32 - (f32::from(super::GameBoard::ROW_COUNT) / 2.0) + 0.5) * Self::LENGTH,
      z: 0.0,
    }
  }

  #[must_use]
  pub fn new_sprite(&self, color: Color, visibility: Visibility) -> SpriteBundle {
    SpriteBundle {
      sprite: Sprite { color, ..default() },
      transform: Transform {
        scale: Vec3::new(
          Self::STICKER_LENGTH,
          Self::STICKER_LENGTH,
          Self::STICKER_LENGTH,
        ),
        translation: self.translation(),
        ..default()
      },
      visibility,
      ..default()
    }
  }

  pub fn shift(mut t: Self, delta_x: Option<i32>, delta_y: Option<i32>) -> Self {
    match delta_x {
      Some(delta) => {
        t.0 += delta;
      }
      None => {}
    }
    match delta_y {
      Some(delta) => {
        t.1 += delta;
      }
      None => {}
    }
    t
  }
}

impl From<[i32; 2]> for Tile {
  fn from([x, y]: [i32; 2]) -> Self {
    Self(x, y)
  }
}

#[must_use]
/// checks whether selected piece(s) collide with either the board, or any other pieces.
pub fn has_collision(
  piece_query: &Query<(&mut Shape, &mut Tile, &mut Transform)>,
  board_query: &Query<&Tile, Without<Shape>>,
) -> bool {
  for (_, block, _) in piece_query {
    if block.0 < 0 {
      return true;
    }
    if block.0 > 9 {
      return true;
    }
    if block.1 < 0 {
      return true;
    }
  }

  for (_, block, _) in piece_query {
    for board_block in board_query {
      if board_block.1 == block.1 && block.0 > 0 && board_block.0 == block.0 - 1 {
        return true;
      }
      if board_block.1 == block.1 && board_block.0 == block.0 + 1 {
        return true;
      }
      if board_block.0 == block.0 && block.1 > 0 && board_block.1 == block.1 - 1 {
        return true;
      }
    }
  }
  false
}

/// a [`System`] which automatically generates a new piece when the previous one is no longer movable.
pub fn spawn_next_piece(
  commands: Commands,
  query: Query<&Shape>,
  mut data: ResMut<GameBoardData>,
  mut piece_queue: ResMut<Queue>,
) {
  if piece_queue.0.len() < Shape::VARIANTS as usize {
    piece_queue
      .0
      .extend(Tetromino::random_n(Shape::VARIANTS as usize));
  }
  if query.is_empty() {
    piece_queue.0.pop_front().unwrap().spawn_movable(commands);
    // player may hold a piece again.
    data.held = false;
  }
}

/// [component][`Component`] which marks a [`Tetromino`] as the **_next_** [`Tetromino`] to fall.
#[derive(Debug, Component)]
pub struct NextTetromino;

impl NextTetromino {
  /// system responsible for clearing the [`NextTetromino`] from [`Entity`](s).
  pub fn clear(mut commands: Commands, query: Query<Entity, With<Self>>) {
    for entity in &query {
      commands.entity(entity).despawn();
    }
  }

  /// [system][`System`] which spawns the next [`Tetromino`] to fall, when necessary.
  pub fn update(
    mut commands: Commands,
    piece_queue: Res<Queue>,
    mut board_data: ResMut<GameBoardData>,
    query: Query<Entity, With<Self>>,
  ) {
    if board_data.next_piece.is_none()
      || piece_queue.0.front().unwrap().shape != board_data.next_piece.unwrap()
    {
      board_data.next_piece = Some(piece_queue.0.front().unwrap().shape);
      for entity in &query {
        commands.entity(entity).despawn();
      }
      let next = piece_queue.0.front().unwrap().shape;
      let blocks = Tetromino::shift_piece(next.shape(), Some(-8), Some(12));
      for i in 0..4 {
        commands
          .spawn(blocks[i].new_sprite(next.color(), Visibility::Visible))
          .insert(NextTetromino);
      }
    }
  }
}
