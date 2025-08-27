use crate::AppState;
use bevy::app::{App, Plugin};
use bevy::color::palettes::css::BLACK;
use bevy::prelude::*;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub struct MainMenuPlugin;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Pause,
    Quit,
}

#[derive(Component)]
struct MainMenuRoot;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), spawn_main_menu)
            .add_systems(Update, (menu_action, keyboard_input))
            .add_systems(OnExit(AppState::Menu), despawn_main_menu);
    }
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_node = Node {
        width: Val::Px(200.0),
        height: Val::Px(40.0),
        margin: UiRect::top(Val::Px(50.0)),
        padding: UiRect::all(Val::Px(15.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let font_reg = asset_server.load("fonts/font_regular.ttf");
    let font_it = asset_server.load("fonts/font_it.ttf");
    let bg_scroll = asset_server.load("buttons/menu_background.png");

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        MainMenuRoot,
        BackgroundColor(Color::from(BLACK)),
        children![(
            ImageNode::from(bg_scroll),
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                width: Val::Px(450.0),
                height: Val::Px(670.0),
                ..default()
            }     ,
            children![
                (
                    Text::new("GAME MENU"),
                    TextFont {
                        font_size: 30.0,
                        font: font_reg.clone(),
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: UiRect::px(0.0, 0.0, 100.0, 100.0),
                        ..default()
                    },
                ),
                (
                    Button,
                    button_node.clone(),
                    MenuButtonAction::Play,
                    children![(
                        Text::new("Start Game"),
                        TextFont {
                            font_size: 25.0,
                            font: font_it.clone(),
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ),]
                ),
                (
                    Button,
                    button_node.clone(),
                    MenuButtonAction::Quit,
                    children![(
                        Text::new("Exit"),
                        TextFont {
                            font_size: 25.0,
                            font: font_it.clone(),
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ),]
                ),
            ]

        )],
    ));
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    std::process::exit(0);
                }
                MenuButtonAction::Play => {
                    menu_state.set(AppState::InGame);
                }
                MenuButtonAction::Pause => {
                    menu_state.set(AppState::Menu);
                }
            }
        }
    }
}

fn keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<NextState<AppState>>,
    current_state: Res<State<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        menu_state.set(AppState::Menu);
    }
    if keyboard.just_pressed(KeyCode::Space) {
        if *current_state.get() != AppState::Paused {
            menu_state.set(AppState::Paused);
        } else {
            menu_state.set(AppState::InGame);
        }
    }
}

fn despawn_main_menu(mut commands: Commands, q: Query<Entity, With<MainMenuRoot>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
