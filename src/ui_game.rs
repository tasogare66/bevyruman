use bevy::prelude::*;

use crate::{AppState, GameFonts, WaveStatus};

pub struct UiGamePlugin;

impl Plugin for UiGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_ui_game)
            .add_systems(
                Update,
                (update_ui_game_system,).run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Component)]
struct TimerText;

fn setup_ui_game(mut commands: Commands, font: Res<GameFonts>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            //background_color: Color::ORANGE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                // Create a TextBundle that has a Text with a single section.
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "0:00.00",
                    TextStyle {
                        // This font is loaded and will be used instead of the default font.
                        font: font.cmn.clone(),
                        font_size: 36.0,
                        ..default()
                    },
                ) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::Center)
                // Set the style of the TextBundle itself.
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    //left: Val::Percent(50.0),
                    //justify_content: JustifyContent::Center,
                    ..default()
                }),
                TimerText,
            ));
        });
}

fn update_ui_game_system(
    wave_status: Res<WaveStatus>,
    mut query: Query<&mut Text, With<TimerText>>,
) {
    for mut text in &mut query {
        let remaining = wave_status.timer.remaining();
        let mins = remaining.as_secs() / 60;
        let secs = remaining.as_secs() % 60;
        let millis = remaining.subsec_millis() / 10;
        text.sections[0].value = format!("{mins}:{secs:0>2}.{millis:0>2}");
    }
}
