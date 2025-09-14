use bevy::prelude::Color;

#[derive(Clone, Copy)]
pub struct TerminalGlyph {
    pub(crate) glyph: u16,
    pub(crate) foreground: [f32; 4],
    pub(crate) background: [f32; 4],
}

impl Default for TerminalGlyph {
    fn default() -> Self {
        Self {
            glyph: 32,
            foreground: {
                let srgba: bevy::color::Srgba = Color::WHITE.into();
                [srgba.red, srgba.green, srgba.blue, srgba.alpha]
            },
            background: {
                let srgba: bevy::color::Srgba = Color::BLACK.into();
                [srgba.red, srgba.green, srgba.blue, 1.0] // Force alpha to 1.0
            },
        }
    }
}
