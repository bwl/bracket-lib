
use crate::{
    consoles::SparseConsole, fonts::FontStore, BTermBuilder, BracketContext, SimpleConsole,
    TerminalLayer,
};
use bevy::{
    math::Vec2,
    prelude::*,
    color::Color,
    image::ImageSampler,
    render::{
        camera::{RenderTarget, ScalingMode, ImageRenderTarget, ClearColorConfig, ViewportOrigin},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
    transform::components::Transform,
};

use super::image_fixer::ImagesToLoad;

#[derive(Component)]
pub struct BracketCamera;

#[derive(Component)]
pub struct TerminalCamera;

#[derive(Component)]
pub struct DisplaySprite;

#[derive(Resource)]
pub struct TerminalImage(pub Handle<Image>);

#[derive(Component)]
pub struct DisplayCamera;

pub(crate) fn load_terminals(
    context: Res<BTermBuilder>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
) {
    let terminal_pixel_size = if context.with_ortho_camera {
        // Calculate terminal dimensions in pixels (80x50 at 8x8 = 640x400)
        let width = 640.0;  // TODO: Calculate from actual terminal size and font
        let height = 400.0;

        // Create render target texture for the terminal with nearest sampling
        let size = Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        };

        let texture_descriptor = TextureDescriptor {
            label: Some("terminal_render_target".into()),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        };

        let mut render_target_image = Image {
            texture_descriptor,
            sampler: ImageSampler::nearest(),
            ..default()
        };
        render_target_image.resize(size);

        let render_target_handle = images.add(render_target_image);

        // Create terminal camera that renders to the texture with exact pixel dimensions
        let image_target = ImageRenderTarget::from(render_target_handle.clone());
        commands.spawn((
            Camera2d,
            GlobalTransform::default(),
            Camera {
                target: RenderTarget::Image(image_target),
                order: 0,
                hdr: false,
            },
            OrthographicProjection {
                scale: 1.0,
                scaling_mode: ScalingMode::Fixed {
                    width: 640.0,
                    height: 400.0,
                },
                near: -1000.0,
                far: 1000.0,
                viewport_origin: ViewportOrigin::Center,
                area: None,
            },
            ClearColorConfig::Custom(Color::BLACK),
            TerminalCamera,
            BracketCamera,
        ));

        // Create display camera that renders the texture to the window
        commands.spawn((
            Camera2d,
            GlobalTransform::default(),
            Camera::default(),
            OrthographicProjection {
                scale: 1.0,
                scaling_mode: ScalingMode::WindowSize,
                near: -1000.0,
                far: 1000.0,
                viewport_origin: ViewportOrigin::Center,
                area: None,
            },
            DisplayCamera,
        ));

        // Store the terminal image handle as a resource
        commands.insert_resource(TerminalImage(render_target_handle));

        Some((width, height, render_target_handle))
    } else {
        None
    };

    // Setup the new context
    let mut new_context = BracketContext::new(context.palette.clone());
    new_context.scaling_mode = context.scaling_mode;

    // Load the fonts
    let mut texture_handles = Vec::<UntypedHandle>::new();
    for font in context.fonts.iter() {
        let texture_handle = asset_server.load(&font.filename);
        let material_handle = materials.add(ColorMaterial::from(texture_handle.clone()));
        texture_handles.push(texture_handle.clone().untyped());
        new_context.fonts.push(FontStore::new(
            texture_handle.clone(),
            material_handle,
            font.chars_per_row,
            font.n_rows,
            font.font_height_pixels,
        ));
    }
    commands.insert_resource(ImagesToLoad(texture_handles));

    // Setup the consoles
    for (idx, terminal) in context.layers.iter().enumerate() {
        match terminal {
            TerminalLayer::Simple {
                font_index,
                width,
                height,
                features,
            } => {
                let mut console = SimpleConsole::new(*font_index, *width, *height);
                console.initialize(&new_context.fonts, &mut meshes, features);
                console.spawn(
                    &mut commands,
                    new_context.fonts[*font_index].material_handle.clone(),
                    idx,
                );
                new_context.terminals.lock().push(Box::new(console));
            }
            TerminalLayer::Sparse {
                font_index,
                width,
                height,
                features,
            } => {
                let mut console = SparseConsole::new(*font_index, *width, *height);
                console.initialize(&new_context.fonts, &mut meshes, features);
                console.spawn(
                    &mut commands,
                    new_context.fonts[*font_index].material_handle.clone(),
                    idx,
                );
                new_context.terminals.lock().push(Box::new(console));
            }
        }
    }

    // Create display quad if we have a render target
    if let Some((width, height, texture_handle)) = terminal_pixel_size {
        // Spawn the display sprite using SpriteBundle
        commands.spawn((
            SpriteBundle {
                texture: texture_handle,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(width, height)),
                    ..default()
                },
                ..default()
            },
            DisplaySprite,
        ));
    }

    // Clean up after the building process
    commands.remove_resource::<BTermBuilder>();
    commands.insert_resource(new_context);
}
