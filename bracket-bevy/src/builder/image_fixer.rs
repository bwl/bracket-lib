use bevy::{
    image::{ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};

#[derive(Resource)]
pub(crate) struct ImagesToLoad(pub(crate) Vec<UntypedHandle>);

pub(crate) fn fix_images(mut fonts: ResMut<ImagesToLoad>, mut images: ResMut<Assets<Image>>) {
    if fonts.0.is_empty() {
        return;
    }

    for (handle, img) in images.iter_mut() {
        let mut to_remove = Vec::new();
        if let Some(i) = fonts.0.iter().enumerate().find(|(_i, h)| h.id() == handle) {
            img.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                label: Some("LeaveItAlone".to_string()),
                address_mode_u: bevy::image::ImageAddressMode::ClampToEdge,
                address_mode_v: bevy::image::ImageAddressMode::ClampToEdge,
                address_mode_w: bevy::image::ImageAddressMode::ClampToEdge,
                mag_filter: bevy::image::ImageFilterMode::Nearest,
                min_filter: bevy::image::ImageFilterMode::Nearest,
                mipmap_filter: bevy::image::ImageFilterMode::Nearest,
                ..Default::default()
            });
            to_remove.push(i.0);
        }
        to_remove.iter().for_each(|i| {
            fonts.0.remove(*i);
        });
    }
}
