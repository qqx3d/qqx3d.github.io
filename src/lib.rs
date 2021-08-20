#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_ref)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(associated_type_bounds)]

mod init;
mod vec;
mod color;
mod polygon;
pub(crate) mod window;

pub use polygon::{Vertex, Shape};
pub use init::{Hint, initialize};
pub use vec::{Vec1, Vec2, Vec3, Vec4};
pub use color::Color;
pub use window::Window;
