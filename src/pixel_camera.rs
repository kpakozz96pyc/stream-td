use bevy::math::FloatOrd;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy::render::camera::{RenderTarget, ImageRenderTarget};
use bevy::render::view::visibility::RenderLayers;
use crate::camera_controls::ControllableCamera;

const LOW_W: u32 = 160;
const LOW_H: u32 = 144;

#[derive(Component)]
struct RotateCube;

#[derive(Resource)]
struct PixelTarget {
    handle: Handle<Image>,
    base: UVec2,
}

#[derive(Component)]
struct PixelScreen;

pub struct PixelCameraPlugin;

 impl Plugin for PixelCameraPlugin {
     fn build(&self, app: &mut App) {
         app
             .add_systems(Startup, (setup_target));
             //.add_systems(Update, (fit_sprite_to_window));
     }
}

fn setup_target(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // создаём низкоразрешённую текстуру под рендер
    let size = Extent3d { width: LOW_W, height: LOW_H, ..default() };
    let mut img = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        default(),
    );
    img.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT;
    let handle = images.add(img);

    commands.insert_resource(PixelTarget { handle: handle.clone(), base: UVec2::new(LOW_W, LOW_H) });

    commands.spawn((
        Camera3d::default(),
        Camera {
            target: RenderTarget::Image(ImageRenderTarget {
                handle: handle.clone(),
                scale_factor: FloatOrd(1.0),
            }),
            clear_color: ClearColorConfig::Custom(Color::NONE),
            order: 0,
            ..default()
        },
        Transform::from_xyz(0.0, 2.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::layer(0),
        ControllableCamera
    ));

    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        Transform::from_xyz(0.0, 2.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::layer(1),
        ControllableCamera,
    ));

    commands.spawn((
        Camera2d::default(),
        Camera {
            order: 1,
            ..default()
        },
    ));

    commands.spawn((
        Sprite {
            image: handle,
            custom_size: Some(Vec2::new(LOW_W as f32, LOW_H as f32)),
            ..default()
        },
        Transform::from_translation(Vec3::ZERO),
        PixelScreen,
    ));
}

fn fit_sprite_to_window(
    windows: Query<&Window>,
    mut q_sprite: Query<(&mut Transform, &mut Sprite), With<PixelScreen>>,
    target: Res<PixelTarget>,
) {
    let Ok(window) = windows.single() else { return; };
    let win_w = window.physical_width() as f32;
    let win_h = window.physical_height() as f32;

    let base_w = target.base.x as f32;
    let base_h = target.base.y as f32;

    let scale_x = (win_w / base_w).floor().max(1.0);
    let scale_y = (win_h / base_h).floor().max(1.0);
    let scale = scale_x.min(scale_y);

    let out_w = base_w * scale;
    let out_h = base_h * scale;

    if let Ok((mut tr, mut sprite)) = q_sprite.single_mut() {
        sprite.custom_size = Some(Vec2::new(out_w, out_h));
        tr.translation.x = (win_w - out_w) * 0.5 - (win_w * 0.5) + out_w * 0.5;
        tr.translation.y = (win_h - out_h) * 0.5 - (win_h * 0.5) + out_h * 0.5;
        tr.translation.z = 0.0;
        tr.scale = Vec3::ONE;
    }
}
