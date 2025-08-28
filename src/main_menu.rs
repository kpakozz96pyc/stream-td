use crate::AppState;
use bevy::app::{App, Plugin};
use bevy::audio::Volume;
use bevy::prelude::*;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub struct MainMenuPlugin;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

#[derive(Component)]
struct MainMenuRoot;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuAssets>()
            .add_systems(OnEnter(AppState::Menu), spawn_main_menu)
            .add_systems(
                Update,
                (menu_action, keyboard_input, button_visuals_and_sounds),
            )
            .add_systems(OnExit(AppState::Menu), despawn_main_menu)
            .add_systems(OnEnter(AppState::Paused), pause_all_animations)
            .add_systems(OnExit(AppState::Paused), resume_all_animations);
    }
}

fn spawn_main_menu(mut commands: Commands, assets: Res<MenuAssets>) {
    let button_node = Node {
        width: Val::Px(200.0),
        height: Val::Px(40.0),
        margin: UiRect::top(Val::Px(50.0)),
        padding: UiRect::all(Val::Px(15.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(0.0)),

        ..default()
    };

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        MainMenuRoot,
        children![(
            ImageNode::from(assets.bg_scroll.clone()),
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                width: Val::Px(450.0),
                height: Val::Px(670.0),
                ..default()
            },
            children![
                (
                    Text::new("GAME MENU"),
                    TextFont {
                        font_size: 30.0,
                        font: assets.font_reg.clone(),
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
                    BorderRadius::all(Val::Px(8.0)),
                    BorderColor(Color::NONE),
                    MenuButtonAction::Play,
                    children![(
                        Text::new("Start Game"),
                        TextFont {
                            font_size: 25.0,
                            font: assets.font_it.clone(),
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ),]
                ),
                (
                    Button,
                    button_node.clone(),
                    BorderColor(Color::NONE),
                    BorderRadius::all(Val::Px(8.0)),
                    MenuButtonAction::Quit,
                    children![(
                        Text::new("Exit"),
                        TextFont {
                            font_size: 25.0,
                            font: assets.font_it.clone(),
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

fn button_visuals_and_sounds(
    mut q: Query<(&Interaction, &mut Node, &mut BorderColor), (Changed<Interaction>, With<Button>)>,
    assets: Res<MenuAssets>,
    mut commands: Commands,
) {
    let hover_col = Color::srgb(0.95, 0.8, 0.35);
    let press_col = Color::srgb(0.9, 0.2, 0.2);

    for (interaction, mut node, mut border_color) in &mut q {
        match *interaction {
            Interaction::Hovered => {
                node.border = UiRect::all(Val::Px(3.0));
                border_color.0 = hover_col;
                commands.spawn((
                    AudioPlayer::new(assets.sfx_hover.clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.9)),
                ));
            }
            Interaction::Pressed => {
                node.border = UiRect::all(Val::Px(3.0));
                border_color.0 = press_col;
                commands.spawn((
                    AudioPlayer::new(assets.sfx_click.clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::Linear(1.0)),
                ));
            }
            Interaction::None => {
                node.border = UiRect::all(Val::Px(0.0));
                border_color.0 = Color::NONE;
            }
        }
    }
}

fn pause_all_animations(mut q: Query<&mut AnimationPlayer>) {
    for mut p in &mut q {
        p.pause_all();
    }
}

fn resume_all_animations(mut q: Query<&mut AnimationPlayer>) {
    for mut p in &mut q {
        p.resume_all();
    }
}

#[derive(Resource)]
struct MenuAssets {
    font_reg: Handle<Font>,
    font_it: Handle<Font>,
    bg_scroll: Handle<Image>,
    sfx_hover: Handle<AudioSource>,
    sfx_click: Handle<AudioSource>,
}

impl FromWorld for MenuAssets {
    fn from_world(world: &mut World) -> Self {
        let server = world.resource::<AssetServer>();
        Self {
            font_reg: server.load("fonts/font_regular.ttf"),
            font_it: server.load("fonts/font_it.ttf"),
            bg_scroll: server.load("buttons/menu_background.png"),
            sfx_hover: server.load("buttons/button_hover.ogg"),
            sfx_click: server.load("buttons/button_click.ogg"),
        }
    }
}
