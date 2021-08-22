#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_ref)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(associated_type_bounds)]
#![feature(try_trait_v2)]
#![feature(control_flow_enum)]

mod vec;
mod color;
mod window;
mod event;

pub(crate) mod init;

pub mod polygon;

pub use init::{Hint, initialize};
pub use vec::{Vec1, Vec2, Vec3, Vec4};
pub use color::Color;
pub use window::{Window, WindowError};
pub use event::{Event, eventloop};
