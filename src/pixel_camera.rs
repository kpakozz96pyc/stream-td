use bevy::image::ImageSampler;
use bevy::math::FloatOrd;
use bevy::prelude::*;
use bevy::render::camera::{CameraOutputMode, ImageRenderTarget, RenderTarget};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy::render::view::visibility::RenderLayers;
use bevy::window::WindowResized;
use crate::camera_controls::ControllableCamera;

const PIXEL_SCALE: u32 = 8;

#[derive(Resource)]
struct PixelCamRefs {
    pixel_cam: Entity,
    pick_cam: Entity,
}

#[derive(Resource)]
struct PixelTarget {
    handle: Handle<Image>,
    base: UVec2,
    scale: u32,
}

#[derive(Component)]
struct PixelScreen;

pub struct PixelCameraPlugin;
#[derive(Component)]
struct PickCamTag;


impl Plugin for PixelCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_pixel_camera)
            .add_systems(Update, sync_pick_camera_to_pixel)
            .add_systems(Update, resize_on_window_change);
    }
}

fn make_image(extent: Extent3d) -> Image {
    let mut img = Image::new_fill(
        extent,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        default(),
        
    );
    img.sampler = ImageSampler::nearest();
    img.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST;
    img
}

fn setup_pixel_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    windows: Query<&Window>,
) {
    let window = windows.single().unwrap();
    let win_width = window.resolution.physical_width();
    let win_height = window.resolution.physical_height();
    let base_w = (win_width.max(1) / PIXEL_SCALE).max(1);
    let base_h = (win_height.max(1) / PIXEL_SCALE).max(1);
    let extent = Extent3d { width: base_w, height: base_h, ..default() };
    let handle = images.add(make_image(extent));

    commands.insert_resource(PixelTarget {
        handle: handle.clone(),
        base: UVec2::new(base_w, base_h),
        scale: PIXEL_SCALE,
    });

    let pixel_cam = commands
        .spawn((
            Camera3d::default(),
            Camera {
                target: RenderTarget::Image(ImageRenderTarget {
                    handle: handle.clone(),
                    scale_factor: FloatOrd(1.0),
                }),
                clear_color: ClearColorConfig::Custom(Color::NONE),
                order: 1,
                ..default()
            },
            Transform::from_xyz(0.0, 2.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
            RenderLayers::layer(0),
            ControllableCamera,
            Name::new("PixelCam"),
        ))
        .id();

    let pick_cam = commands
        .spawn((
            Camera3d::default(),
            Camera {
                output_mode: CameraOutputMode::Skip,
                order: -10,
                ..default()
            },
            Transform::default(),
            RenderLayers::layer(0),
            Name::new("PickCam"),
            PickCamTag
        ))
        .id();

    commands.spawn((
        Camera3d::default(),
        Camera { order: 0, ..default() },
        Transform::from_xyz(0.0, 2.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::layer(1),
        ControllableCamera
    ));

    commands.insert_resource(PixelCamRefs { pixel_cam, pick_cam });

    commands.spawn((Camera2d::default(), Camera { order: 1, ..default() }));

    commands.spawn((
        Sprite {
            image: handle,
            custom_size: Some(Vec2::new(window.width(), window.height())),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        PixelScreen,
    ));
}

use bevy::render::camera::Projection;

fn sync_pick_camera_to_pixel(
    refs: Option<Res<PixelCamRefs>>,
    src_q: Query<(&GlobalTransform, &Projection), (With<Camera3d>, Without<PickCamTag>)>,
    mut dst_q: Query<(&mut Transform, &mut Projection), With<PickCamTag>>,
) {
    let Some(refs) = refs else { return; };

    if let Ok((src_gt, src_proj)) = src_q.get(refs.pixel_cam) {
        if let Ok((mut dst_t, mut dst_proj)) = dst_q.get_mut(refs.pick_cam) {
            *dst_t = Transform {
                translation: src_gt.translation(),
                rotation: src_gt.rotation(),
                scale: Vec3::ONE,
            };
            *dst_proj = src_proj.clone();
        }
    }
}

fn resize_on_window_change(
    mut evw: EventReader<WindowResized>,
    mut images: ResMut<Assets<Image>>,
    mut target: ResMut<PixelTarget>,
    mut q_sprite: Query<&mut Sprite, With<PixelScreen>>,
    windows: Query<&Window>,
) {
    if evw.is_empty() {
        return;
    }
    evw.clear();

    let window = windows.single().unwrap();

    if let Ok(mut sprite) = q_sprite.single_mut() {
        sprite.custom_size = Some(Vec2::new(window.width(), window.height()));
    }

    let win_w = window.resolution.physical_width().max(1);
    let win_h = window.resolution.physical_height().max(1);
    let new_base_w = (win_w / target.scale).max(1);
    let new_base_h = (win_h / target.scale).max(1);

    if target.base.x != new_base_w || target.base.y != new_base_h {
        target.base = UVec2::new(new_base_w, new_base_h);
        if let Some(img) = images.get_mut(&target.handle) {
            let new_extent = Extent3d { width: new_base_w, height: new_base_h, ..default() };
            img.resize(new_extent);
        }
    }
}
