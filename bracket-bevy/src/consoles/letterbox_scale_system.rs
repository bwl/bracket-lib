use bevy::{
    prelude::*,
    window::WindowResized,
};
use crate::{
    consoles::ScreenScaler,
    TerminalImage,
    DisplaySprite,
    context::BracketContext,
};

pub fn letterbox_scale_system(
    mut resize_events: EventReader<WindowResized>,
    windows: Query<&Window>,
    mut scaler: ResMut<ScreenScaler>,
    _terminal_image: Res<TerminalImage>,
    context: Res<BracketContext>,
    mut display_query: Query<
        (&mut Transform, &mut Sprite),
        With<DisplaySprite>
    >,
) {
    if resize_events.is_empty() {
        return;
    }

    // Get the current window size
    if let Ok(window) = windows.single() {
        let window_size = Vec2::new(
            window.resolution.width(),
            window.resolution.height()
        );
        scaler.set_screen_size(window_size.x, window_size.y);
    }

    // Use fixed terminal dimensions for render-to-texture (640x400)
    let terminal_dims = (640.0, 400.0);
    
    // Get largest font size from context
    let largest_font = context.largest_font();
    
    // Recalculate scaling for pixel-perfect mode
    scaler.recalculate(terminal_dims, largest_font, crate::TerminalScalingMode::PixelPerfect);

    // Update display sprite transform for letterboxing
    if let Ok((mut transform, mut sprite)) = display_query.single_mut() {
        let scale_factor = scaler.get_scale_factor() as f32;
        
        // Update sprite size to scaled terminal size
        let scaled_size = Vec2::new(
            terminal_dims.0 * scale_factor,
            terminal_dims.1 * scale_factor,
        );
        sprite.custom_size = Some(scaled_size);

        // Calculate centered position with letterboxing using scaler gutters
        let top_left = scaler.top_left();
        let x_center = top_left.0 + scaled_size.x / 2.0;
        let y_center = top_left.1 + scaled_size.y / 2.0;

        transform.translation = Vec3::new(x_center, y_center, 0.0);
        transform.scale = Vec3::new(1.0, 1.0, 1.0); // Scale handled by custom_size
    }

    // Clear the resize events
    resize_events.clear();
}