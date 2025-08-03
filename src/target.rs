use bevy::asset::Assets;
use bevy::color::{Color, Srgba};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::*;
use crate::world::Game;

pub struct TargetPlugin;

impl Plugin for TargetPlugin{
    fn build(&self,app: &mut App){
        app.register_type::<Target>()
            .add_systems(Update, (spawn_targets, move_targets));

        return;
    }
}


#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target{
    pub speed: f32,
    pub health: f32,
}

fn spawn_targets(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: Query<&mut Game>, 
time: Res<Time>){
    let mut game = game.single_mut().unwrap();
    game.target_spawn_timer.tick(time.delta());
    if game.target_spawn_timer.just_finished(){
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
            MeshMaterial3d(materials.add(Color::from(Srgba::new(0.3, 0.3, 0.6, 0.8)))),
            Transform::from_xyz(-2.0, 0.25, 2.0),
        )).insert(Target{speed: 0.3, health: 100.0}).insert(Name::new("Target"));
    }
}

fn move_targets(mut targets: Query<(&mut Transform, &Target)>, time: Res<Time>){
    for (mut transform, target) in targets.iter_mut(){
        transform.translation.x += target.speed*time.delta_secs();
    }
}

