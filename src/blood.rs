use bevy::prelude::*;
use bevy::pbr::{NotShadowCaster, MeshMaterial3d};
use bevy::math::primitives::Rectangle;
use bevy::color::Srgba;
use rand::Rng;

#[derive(Event)]
pub struct SpawnBlood {
    pub pos: Vec3,
}

#[derive(Component)]
struct BloodSplash {
    timer: Timer,
}


const BLOOD_TEX: &str = "png/blood_sprite.png";

fn spawn_blood_splash(
    mut ev: EventReader<SpawnBlood>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    for SpawnBlood { pos } in ev.read() {

        let mut rng = rand::rng();
        let scale = rng.random_range(1.0..1.6);
        let y_offset = 0.01;

        let mesh = meshes.add(Mesh::from(Rectangle::new(1.0, 1.0)));

        let material = materials.add(StandardMaterial {
            base_color_texture: Some(assets.load(BLOOD_TEX)),
            base_color: Color::Srgba(Srgba::new(1.0, 1.0, 1.0, 1.0)),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });

        // Position the blood splash flat on the floor with a slight rotation for randomness
        let rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2) * 
                       Quat::from_rotation_z(rng.random_range(0.0..std::f32::consts::TAU));
        
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(*pos + Vec3::Y * y_offset)
                .with_rotation(rotation)
                .with_scale(Vec3::splat(scale)),
            NotShadowCaster,
            BloodSplash {
                timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
        ));
    }
}


fn fade_and_despawn_splashes(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut BloodSplash, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (e, mut s, mat_h) in &mut q {
        s.timer.tick(time.delta());
        let t = 1.0 - s.timer.fraction(); // 1 -> 0

        if let Some(mat) = materials.get_mut(&mat_h.0) {
            if let Color::Srgba(mut c) = mat.base_color {
                c.set_alpha(t);
                mat.base_color = Color::Srgba(c);
            }
        }

        if s.timer.finished() {
            commands.entity(e).despawn();
        }
    }
}

pub struct BloodPlugin;

impl Plugin for BloodPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBlood>()
            .add_systems(Update, (spawn_blood_splash, fade_and_despawn_splashes));
    }
}