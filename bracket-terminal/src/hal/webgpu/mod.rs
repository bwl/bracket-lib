//! Provides wgpu support back-end.

mod platform;
pub use platform::*;
mod init;
pub use init::*;
mod font;
pub use font::*;
mod shader;
pub use shader::*;
mod backend;
pub use backend::*;
mod mainloop;
pub use mainloop::*;
// VirtualKeyCode is now provided by the parent hal module
mod backing;
pub(crate) use backing::*;
mod framebuffer;
pub(crate) use framebuffer::*;
mod quadrender;

pub fn log(s: &str) {
    println!("{}", s);
}
