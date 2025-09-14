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
        let base_y = glyph / self.chars_per_row;
        let scale_x = 1.0 / self.chars_per_row as f32;
        let scale_y = 1.0 / self.n_rows as f32;
        [
            base_x as f32 * scale_x,
            base_y as f32 * scale_y,
            (base_x + 1) as f32 * scale_x,
            (base_y + 1) as f32 * scale_y,
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
                // Original scaling behavior
                let aspect_ratio = terminal_pixel_size.0 / terminal_pixel_size.1;
                let perfect_height = self.screen.0 / aspect_ratio;
                if perfect_height <= self.screen.1 {
                    self.y_gutter = self.screen.1 - perfect_height;
                    self.x_gutter = self.desired_gutter;
                } else {
                    let perfect_width = self.screen.1 * aspect_ratio;
                    self.x_gutter = self.screen.0 - perfect_width;
                    self.y_gutter = self.desired_gutter;
                }
                self.scale_factor = 1;
            }
        }
    }

    fn recalculate_pixel_perfect(&mut self, terminal_pixel_size: (f32, f32)) {
        // Calculate the maximum integer scale factor that fits in the window
        let max_scale_x = (self.screen.0 / terminal_pixel_size.0).floor() as i32;
        let max_scale_y = (self.screen.1 / terminal_pixel_size.1).floor() as i32;

        // Use the smaller scale factor to ensure both dimensions fit
        self.scale_factor = (max_scale_x.min(max_scale_y)).max(1);

        // Calculate the actual terminal size at this integer scale
        let scaled_terminal_width = terminal_pixel_size.0 * self.scale_factor as f32;
        let scaled_terminal_height = terminal_pixel_size.1 * self.scale_factor as f32;

        // Center the terminal with letterboxing
        self.x_gutter = self.screen.0 - scaled_terminal_width;
        self.y_gutter = self.screen.1 - scaled_terminal_height;
    }

    pub(crate) fn top_left(&self) -> (f32, f32) {
        let x = 0.0 - (self.screen.0 / 2.0) + (self.x_gutter as f32 / 2.0);
        let y = 0.0 - (self.screen.1 / 2.0) + (self.y_gutter as f32 / 2.0);

        if self.is_pixel_perfect_mode() {
            // In pixel-perfect mode, ensure top-left is pixel-aligned
            (x.round(), y.round())
        } else {
            (x, y)
        }
    }

    pub(crate) fn calc_step(&self, width: i32, height: i32, font_size: (f32, f32)) -> (f32, f32) {
        if self.is_pixel_perfect_mode() {
            // For pixel-perfect scaling, use the integer-scaled font size
            (
                font_size.0 * self.scale_factor as f32,
                font_size.1 * self.scale_factor as f32,
            )
        } else {
            // Original fractional scaling behavior
            (
                (self.screen.0 - self.x_gutter as f32) / width as f32,
                (self.screen.1 - self.y_gutter as f32) / height as f32,
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
    ) -> (i32, i32) {
        let step = self.calc_step(width, height, (8.0, 8.0)); // Default font size for mouse calc
        let step_pos = (
            (pos.0 / step.0) + (width as f32 / 2.0),
            (pos.1 / step.1) + (height as f32 / 2.0),
        );
        (
            i32::clamp(step_pos.0 as i32, 0, width - 1),
            i32::clamp(height as i32 - step_pos.1 as i32 - 1, 0, height - 1),
        )
    }

    pub fn get_scale_factor(&self) -> i32 {
        self.scale_factor
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub(crate) fn default_gutter_size() -> f32 {
    // Reduced gutter to minimize rendering artifacts
    2.0
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub(crate) fn default_gutter_size() -> f32 {
    // Testing showed that an 8-pixel gutter is enough to fix
    // Big Sur and Windows 11.
    0.0
}
