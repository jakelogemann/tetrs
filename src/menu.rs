//! menu system(s).
use crate::prelude::*;

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct OnGamePausedMenuScreen;

#[derive(Component)]
pub struct OnGameOverMenuScreen;

/// The action to take when a menu button is clicked.
#[derive(Debug, Component)]
pub enum MenuButtonAction {
  /// Start the game.
  StartGame,
  /// Restart the game.
  RestartGame,
  /// Go back to the main menu.
  BackToMainMenu,
  /// Resume the game.
  ResumeGame,
  /// Quit the game.
  Quit,
}

pub fn setup_main_menu(mut commands: Commands, game_assets: Res<GameAssets>) {
  commands
    .spawn((
      NodeBundle {
        background_color: palette::BACKGROUND.into(),
        style: Style {
          size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          ..default()
        },
        ..default()
      },
      OnMainMenuScreen,
    ))
    .insert(Name::new("Main Menu"))
    .with_children(|parent| {
      parent
        .spawn(NodeBundle {
          style: Style {
            size: Size::new(Val::Percent(33.0), Val::Percent(50.0)),
            margin: UiRect::vertical(Val::Px(25.0)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
          },
          background_color: palette::MENU_BACKGROUND.into(),
          ..default()
        })
        .with_children(|parent| {
          parent.spawn(ImageBundle {
            image: game_assets.game_icon(),
            style: Style {
              margin: UiRect::vertical(Val::Px(10.0)),
              size: Size::new(Val::Px(72.0), Val::Px(72.0)),
              ..default()
            },
            ..default()
          });
          parent.spawn(
            TextBundle::from_section(
              crate::GAME_NAME.to_string(),
              TextStyle {
                font: game_assets.bold_font(),
                font_size: 50.0,
                color: palette::TEXT,
              },
            )
            .with_style(Style {
              margin: UiRect::bottom(Val::Percent(10.0)),
              ..default()
            }),
          );
          parent
            .spawn((
              ButtonBundle {
                style: Style {
                  size: Size::new(Val::Percent(65.0), Val::Percent(65.0)),
                  margin: UiRect::vertical(Val::Px(10.0)),
                  justify_content: JustifyContent::Center,
                  align_items: AlignItems::Center,
                  ..default()
                },
                background_color: palette::GREEN.into(),
                ..default()
              },
              MenuButtonAction::StartGame,
            ))
            .with_children(|parent| {
              parent.spawn(TextBundle::from_section(
                "Start New Game",
                TextStyle {
                  font: game_assets.bold_font(),
                  font_size: 25.0,
                  color: palette::BASE,
                },
              ));
            });
          parent
            .spawn((
              ButtonBundle {
                style: Style {
                  size: Size::new(Val::Percent(35.0), Val::Percent(50.0)),
                  margin: UiRect::vertical(Val::Px(15.0)),
                  justify_content: JustifyContent::Center,
                  align_items: AlignItems::Center,
                  ..default()
                },
                background_color: palette::RED.into(),
                ..default()
              },
              MenuButtonAction::Quit,
            ))
            .with_children(|parent| {
              parent.spawn(TextBundle::from_section(
                "Quit",
                TextStyle {
                  font: game_assets.bold_font(),
                  font_size: 20.0,
                  color: palette::BASE,
                },
              ));
            });
        });
    });
}

pub fn setup_game_over_menu(mut commands: Commands, game_assets: Res<GameAssets>) {
  commands
    .spawn((
      NodeBundle {
        style: Style {
          size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          ..default()
        },
        ..default()
      },
      OnGameOverMenuScreen,
    ))
    .with_children(|parent| {
      parent
        .spawn(NodeBundle {
          style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
          },
          background_color: palette::MENU_BACKGROUND.into(),
          ..default()
        })
        .with_children(|parent| {
          parent.spawn(
            TextBundle::from_section(
              "Game Over",
              TextStyle {
                font: game_assets.bold_font(),
                font_size: 25.0,
                color: Color::rgb(0.9, 0.9, 0.9),
              },
            )
            .with_style(Style {
              margin: UiRect::all(Val::Px(20.0)),
              ..default()
            }),
          );

          parent
            .spawn((
              ButtonBundle {
                style: Style {
                  size: Size::new(Val::Px(90.0), Val::Px(30.0)),
                  margin: UiRect::all(Val::Px(10.0)),
                  justify_content: JustifyContent::Center,
                  align_items: AlignItems::Center,
                  ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
              },
              MenuButtonAction::BackToMainMenu,
            ))
            .with_children(|parent| {
              parent.spawn(TextBundle::from_section(
                "Main Menu",
                TextStyle {
                  font: game_assets.bold_font(),
                  font_size: 20.0,
                  color: Color::rgb(0.9, 0.9, 0.9),
                },
              ));
            });

          parent
            .spawn((
              ButtonBundle {
                style: Style {
                  size: Size::new(Val::Px(90.0), Val::Px(30.0)),
                  margin: UiRect::all(Val::Px(10.0)),
                  justify_content: JustifyContent::Center,
                  align_items: AlignItems::Center,
                  ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
              },
              MenuButtonAction::RestartGame,
            ))
            .with_children(|parent| {
              parent.spawn(TextBundle::from_section(
                "Restart",
                TextStyle {
                  font: game_assets.bold_font(),
                  font_size: 20.0,
                  color: Color::rgb(0.9, 0.9, 0.9),
                },
              ));
            });
        });
    });
}

pub fn setup_game_paused_menu(mut commands: Commands, game_assets: Res<GameAssets>) {
  commands
    .spawn((
      NodeBundle {
        style: Style {
          size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          ..default()
        },
        ..default()
      },
      OnGamePausedMenuScreen,
    ))
    .with_children(|parent| {
      parent
        .spawn(NodeBundle {
          style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
          },
          background_color: palette::MENU_BACKGROUND.into(),
          ..default()
        })
        .with_children(|parent| {
          parent.spawn(
            TextBundle::from_section(
              "Game Paused",
              TextStyle {
                font: game_assets.bold_font(),
                font_size: 25.0,
                color: Color::rgb(0.9, 0.9, 0.9),
              },
            )
            .with_style(Style {
              margin: UiRect::all(Val::Px(20.0)),
              ..default()
            }),
          );

          let bb = ButtonBundle {
            style: Style {
              size: Size::new(Val::Px(90.0), Val::Px(30.0)),
              margin: UiRect::all(Val::Px(10.0)),
              justify_content: JustifyContent::Center,
              align_items: AlignItems::Center,
              ..default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
          };

          parent
            .spawn((bb.clone(), MenuButtonAction::BackToMainMenu))
            .with_children(|parent| {
              parent.spawn(game_assets.button_text_bundle("Main Menu"));
            });

          parent
            .spawn((bb.clone(), MenuButtonAction::RestartGame))
            .with_children(|parent| {
              parent.spawn(game_assets.button_text_bundle("Restart"));
            });

          parent
            .spawn((bb.clone(), MenuButtonAction::ResumeGame))
            .with_children(|parent| {
              parent.spawn(game_assets.button_text_bundle("Resume"));
            });
        });
    });
}

pub fn click_button(
  mut interaction_query: Query<
    (&Interaction, &MenuButtonAction),
    (Changed<Interaction>, With<Button>),
  >,
  mut app_state: ResMut<NextState<AppState>>,
  mut game_state: ResMut<NextState<GameState>>,
  mut exit: EventWriter<AppExit>,
) {
  for (interaction, menu_button_action) in &mut interaction_query {
    match *interaction {
      Interaction::Clicked => {
        info!("{:?} button clicked", menu_button_action);
        match menu_button_action {
          MenuButtonAction::StartGame => {
            app_state.set(AppState::InGame);
            game_state.set(GameState::Playing);
          }
          MenuButtonAction::RestartGame => {
            app_state.set(AppState::InGame);
            game_state.set(GameState::Restarting);
          }
          MenuButtonAction::BackToMainMenu => {
            println!("{:?}", app_state.0);
            app_state.set(AppState::MainMenu);
            game_state.set(GameState::Quitting);
          }
          MenuButtonAction::ResumeGame => {
            game_state.set(GameState::Playing);
          }
          MenuButtonAction::Quit => {
            exit.send_default();
          }
        }
      }
      _ => {}
    }
  }
}
