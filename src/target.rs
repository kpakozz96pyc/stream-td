use bevy::asset::Assets;
use bevy::color::{Color, Srgba};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::*;


pub struct TargetPlugin;

impl Plugin for TargetPlugin{
    fn build(&self,app: &mut App){
        app.register_type::<Target>()
            .add_systems(Startup, spawn_targets)
            .add_systems(Update, move_targets);
        
        return;
    }
}


#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target{
    pub speed: f32,
    pub health: f32,
}

fn spawn_targets(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>){
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::from(Srgba::new(0.3, 0.3, 0.6, 0.8)))),
        Transform::from_xyz(-2.0, 0.25, 2.0),
    )).insert(Target{speed: 0.5, health: 100.0}).insert(Name::new("Target1"));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::from(Srgba::new(0.3, 0.3, 0.6, 0.8)))),
        Transform::from_xyz(-4.0, 0.25, 2.0),
    )).insert(Target{speed: 0.3, health: 100.0}).insert(Name::new("Target2"));
}

fn move_targets(mut targets: Query<(&mut Transform, &Target)>, time: Res<Time>){
    for (mut transform, target) in targets.iter_mut(){
        transform.translation.x += target.speed*time.delta_secs();
    }
}