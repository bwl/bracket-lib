use bevy::prelude::Resource;

pub(crate) struct FontScaler {
    chars_per_row: u16,
    n_rows: u16,
    font_height_pixels: (f32, f32),
}

impl FontScaler {
    pub(crate) fn new(chars_per_row: u16, n_rows: u16, font_height_pixels: (f32, f32)) -> Self {
        Self {
            chars_per_row,
            n_rows,
            font_height_pixels,
        }
    }

    pub(crate) fn texture_coords(&self, glyph: u16) -> [f32; 4] {
        let base_x = glyph % self.chars_per_row;
        let base_y = glyph / self.n_rows;
        let scale_x = 1.0 / self.chars_per_row as f32;
        let scale_y = 1.0 / self.n_rows as f32;
        let nudge_pixel_h = (1.0 / (self.chars_per_row as f32 * self.font_height_pixels.0)) / 20.0;
        let nudge_pixel_v = (1.0 / (self.n_rows as f32 * self.font_height_pixels.1)) / 20.0;
        [
            (base_x as f32 * scale_x) + nudge_pixel_h,
            (base_y as f32 * scale_y) + nudge_pixel_v,
            ((base_x + 1) as f32 * scale_x) - nudge_pixel_h,
            ((base_y + 1) as f32 * scale_y) - nudge_pixel_v,
        ]
    }
}

#[derive(Resource)]
pub(crate) struct ScreenScaler {
    pub(crate) screen: (f32, f32),
    desired_gutter: f32,
    x_gutter: f32,
    y_gutter: f32,
    scale_factor: i32,
    pixel_perfect_mode: bool,
}

impl Default for ScreenScaler {
    fn default() -> Self {
        let desired_gutter = default_gutter_size();
        Self {
            screen: (0.0, 0.0),
            desired_gutter,
            x_gutter: desired_gutter / 2.0,
            y_gutter: desired_gutter / 2.0,
            scale_factor: 1,
            pixel_perfect_mode: false,
        }
    }
}

impl ScreenScaler {
    pub(crate) fn new(desired_gutter: f32) -> Self {
        Self {
            screen: (0.0, 0.0),
            desired_gutter,
            x_gutter: desired_gutter / 2.0,
            y_gutter: desired_gutter / 2.0,
            scale_factor: 1,
            pixel_perfect_mode: false,
        }
    }

    pub(crate) fn set_screen_size(&mut self, width: f32, height: f32) {
        self.screen = (width, height);
        self.x_gutter = self.desired_gutter / 2.0;
        self.y_gutter = self.desired_gutter / 2.0;
    }

    pub(crate) fn recalculate(
        &mut self,
        terminal_pixel_size: (f32, f32),
        largest_font: (f32, f32),
        scaling_mode: crate::TerminalScalingMode,
    ) {
        use crate::TerminalScalingMode;

        match scaling_mode {
            TerminalScalingMode::PixelPerfect => {
                self.pixel_perfect_mode = true;
                self.recalculate_pixel_perfect(terminal_pixel_size);
            }
            _ => {
                self.pixel_perfect_mode = false;
                self.scale_factor = 1;
                let aspect_ratio = terminal_pixel_size.0 / terminal_pixel_size.1;
                let perfect_height = (self.screen.0 / aspect_ratio)
                    - (self.screen.1 as u32 % largest_font.1 as u32) as f32;
                if perfect_height < self.screen.1 {
                    self.y_gutter = self.screen.1 - perfect_height;
                    self.x_gutter = self.desired_gutter;
                } else {
                    let perfect_width = (self.screen.1 * aspect_ratio)
                        - (self.screen.0 as u32 % largest_font.0 as u32) as f32;
                    self.x_gutter = self.screen.0 - perfect_width;
                    self.y_gutter = self.desired_gutter;
                }
            }
        }
    }

    fn recalculate_pixel_perfect(&mut self, terminal_pixel_size: (f32, f32)) {
        let max_scale_x = (self.screen.0 / terminal_pixel_size.0).floor() as i32;
        let max_scale_y = (self.screen.1 / terminal_pixel_size.1).floor() as i32;
        self.scale_factor = (max_scale_x.min(max_scale_y)).max(1);

        let scaled_terminal_width = terminal_pixel_size.0 * self.scale_factor as f32;
        let scaled_terminal_height = terminal_pixel_size.1 * self.scale_factor as f32;

        self.x_gutter = self.screen.0 - scaled_terminal_width;
        self.y_gutter = self.screen.1 - scaled_terminal_height;
    }

    pub(crate) fn top_left(&self) -> (f32, f32) {
        let x = 0.0 - (self.screen.0 / 2.0) + (self.x_gutter / 2.0);
        let y = 0.0 - (self.screen.1 / 2.0) + (self.y_gutter / 2.0);

        if self.is_pixel_perfect_mode() {
            (x.round(), y.round())
        } else {
            (x, y)
        }
    }

    pub(crate) fn calc_step(&self, width: i32, height: i32, font_size: (f32, f32)) -> (f32, f32) {
        if self.is_pixel_perfect_mode() {
            (
                font_size.0 * self.scale_factor as f32,
                font_size.1 * self.scale_factor as f32,
            )
        } else {
            (
                (self.screen.0 - self.x_gutter) / width as f32,
                (self.screen.1 - self.y_gutter) / height as f32,
            )
        }
    }

    pub(crate) fn is_pixel_perfect_mode(&self) -> bool {
        self.pixel_perfect_mode
    }

    pub(crate) fn available_size(&self) -> (f32, f32) {
        (self.screen.0 - self.x_gutter, self.screen.1 - self.y_gutter)
    }

    pub(crate) fn calc_mouse_position(
        &self,
        pos: (f32, f32),
        width: i32,
        height: i32,
        font_size: (f32, f32),
    ) -> (i32, i32) {
        let step = self.calc_step(width, height, font_size);
        let step_pos = (
            (pos.0 / step.0) + (width as f32 / 2.0),
            (pos.1 / step.1) + (height as f32 / 2.0),
        );
        (
            i32::clamp(step_pos.0 as i32, 0, width - 1),
            i32::clamp(step_pos.1 as i32, 0, height - 1),
        )
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub(crate) fn default_gutter_size() -> f32 {
    2.0
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub(crate) fn default_gutter_size() -> f32 {
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TerminalScalingMode;

    #[test]
    fn texture_coords_apply_nudge() {
        let scaler = FontScaler::new(16, 16, (8.0, 8.0));
        let coords = scaler.texture_coords(0);
        assert!(coords[0] > 0.0);
        assert!(coords[2] < 1.0);
        assert!(coords[2] > coords[0]);
    }

    #[test]
    fn pixel_perfect_scale_factor_creates_integer_steps() {
        let mut scaler = ScreenScaler::new(0.0);
        scaler.set_screen_size(1920.0, 1080.0);
        scaler.recalculate(
            (640.0, 400.0),
            (8.0, 8.0),
            TerminalScalingMode::PixelPerfect,
        );
        assert_eq!(scaler.scale_factor, 2);
        let step = scaler.calc_step(80, 50, (8.0, 8.0));
        assert_eq!(step, (16.0, 16.0));
    }

    #[test]
    fn stretch_mode_preserves_aspect_ratio() {
        let mut scaler = ScreenScaler::new(0.0);
        scaler.set_screen_size(1280.0, 720.0);
        scaler.recalculate((640.0, 400.0), (8.0, 8.0), TerminalScalingMode::Stretch);
        let step = scaler.calc_step(80, 50, (8.0, 8.0));
        assert!((step.0 - step.1).abs() < 0.01);
    }
}
