use crate::{
    logic,
    states::{AppState, GameVolue},
};
use bevy::{app::AppExit, prelude::*};
use bevy_pkv::PkvStore;

use super::{MenuButtonAction, NORMAL_BUTTON, TEXT_COLOR};

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
pub struct OnBoardSetupScreen;

pub fn board_setup_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut pkv: ResMut<PkvStore>,
) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnBoardSetupScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(70.0),
                        height: Val::Percent(70.0),
                        align_items: AlignItems::Default,
                        justify_content: JustifyContent::Default,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    let white = Color::hex("eeeed2").unwrap();
                    let black = Color::hex("769656").unwrap();
                    for y in 0..4 {
                        for x in 0..4 {
                            let color = if (x + y) % 2 == 0 { white } else { black };
                            parent.spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: color.into(),
                                ..default()
                            });
                        }
                    }
                });
        });
}
