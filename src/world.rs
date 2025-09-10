use bevy::app::{App, Plugin, Startup};
use bevy::asset::Assets;
use bevy::color::{Color, Srgba};
use bevy::pbr::{MeshMaterial3d, PointLight, StandardMaterial};
use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Game {
    pub target_spawn_timer: Timer,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self,app: &mut App){
        app
            .register_type::<Game>()
            .add_systems(Startup, (spawn_light, spawn_scene));

        return;
    }
}

fn spawn_light(mut commands: Commands){
    commands.spawn((PointLight {
        shadows_enabled: true,
        ..default()
    },
                    Transform::from_xyz(5.0, 5.0, 5.0),
    )).insert(Name::new("Sun"));
}

fn spawn_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>){
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::from(Srgba::new(0.3, 0.5, 0.3, 1.0)))),
    ))
        .insert(Name::new("Scene"))
        .insert(Game {target_spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating)});
}