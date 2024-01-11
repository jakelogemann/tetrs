//! scoring system(s) and display.
use crate::prelude::*;

/// game's score data.
#[derive(Resource, Default, Debug)]
pub struct Score {
  /// score points earned.
  pub points: u32,
  /// lines of blocks cleared.
  pub lines: u32,
}

impl Score {
  /// sets the score to zero.
  pub fn reset(mut score: ResMut<Self>) {
    score.lines = 0;
    score.points = 0;
  }
  /// updates the score based on the number of lines cleared.
  pub fn update(&mut self, rows_cleared: usize) {
    let points = match rows_cleared {
      0 => 0,
      1 => 40,
      2 => 100,
      3 => 300,
      4 => 1200,
      _ => panic!("impossible number of rows cleared: {rows_cleared}"),
    };
    self.lines += rows_cleared as u32;
    self.points += points;
    info!(
      "player cleared {} line(s); earning {} points. total={} for {} lines.",
      rows_cleared, points, self.points, self.lines
    );
  }
  /// outputs the current score as a string.
  pub fn score_text(&self) -> String {
    format!("Score: {}\n", self.points)
  }
  /// outputs the current number of lines cleared as a string.
  pub fn lines_text(&self) -> String {
    format!("Lines: {}\n", self.lines)
  }
}

#[derive(Component)]
pub struct Scoreboard;

impl Scoreboard {
  pub(super) fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(Score::default());
    commands
      .spawn(NodeBundle {
        style: Style {
          size: Size::new(Val::Px(220.0), Val::Px(50.0)),
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          flex_direction: FlexDirection::Column,
          position_type: PositionType::Absolute,
          position: UiRect {
            left: Val::Percent(5.0),
            top: Val::Percent(10.0),
            ..default()
          },
          ..default()
        },
        ..default()
      })
      .with_children(|parent| {
        let mut sections = vec![];
        sections.push(TextSection {
          value: "Score:\n".to_string(),
          style: TextStyle {
            font: assets.load("fonts/FiraSans-Bold.ttf"),
            font_size: 35.0,
            color: palette::SUBTEXT1,
          },
        });
        sections.push(TextSection {
          value: "Lines:\n".to_string(),
          style: TextStyle {
            font: assets.load("fonts/FiraSans-Bold.ttf"),
            font_size: 25.0,
            color: palette::SUBTEXT0,
          },
        });
        sections.push(TextSection {
          value: "Held:\n".to_string(),
          style: TextStyle {
            font: assets.load("fonts/FiraSans-Bold.ttf"),
            font_size: 25.0,
            color: palette::SUBTEXT0,
          },
        });
        parent
          .spawn(TextBundle::from_sections(sections).with_style(Style {
            flex_direction: FlexDirection::Row,
            ..default()
          }))
          .insert(Scoreboard);
      })
      .insert(Name::new("Scoreboard"));
  }

  pub(super) fn update(
    score: Res<Score>,
    data: Res<GameBoardData>,
    mut query: Query<&mut Text, With<Self>>,
  ) {
    for mut text in query.iter_mut() {
      text.sections[0].value = score.score_text();
      text.sections[1].value = score.lines_text();
      text.sections[2].value = data
        .held_piece
        .map(|pc| format!("Held: {:?}\n", pc))
        .unwrap_or_default();
    }
  }
}
