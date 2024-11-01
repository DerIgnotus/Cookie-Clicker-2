use bevy::prelude::*;
use bevy::window::PrimaryWindow;
mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (check_click, update_score))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        ..default()
    };

    commands.spawn((Camera2dBundle::default(), components::Cookie {
        score: 0,
        give_amount: 1.0,
    }));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("Clicker.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Cookies: 0", text_style.clone()),
            transform: Transform::from_translation(Vec3::new(0.0, 325.0, 0.0)),
            ..default()
        },
        components::ScoreText,
    ));
}

fn check_click(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut cookie_query: Query<&mut components::Cookie>
) {
    for button in buttons.get_just_pressed() {
        if button == &MouseButton::Left {
            if check_position(&q_windows) {
                cookie_pressed(&mut cookie_query);
            }
        }
    }
}

fn cookie_pressed(cookie_query: &mut Query<&mut components::Cookie>) {
    for mut cookie in cookie_query.iter_mut() {
        cookie.score += cookie.give_amount as u32;
    }
}

fn check_position(q_windows: &Query<&Window, With<PrimaryWindow>>) -> bool {
    if let Some(position) = q_windows.single().cursor_position() {
        if position.x > 540.00 && position.x < 740.00 && position.y > 260.00 && position.y < 460.00
        {
            return true;
        }
    }

    return false;
}

fn update_score(mut query: Query<&mut Text, With<components::ScoreText>>, cookie_query: Query<&components::Cookie>) { 
    for mut text in query.iter_mut() {
        for cookie in cookie_query.iter() {
            text.sections[0].value = format!("Cookies: {:?}", cookie.score);
        }
    }
}
