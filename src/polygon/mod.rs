pub mod separate;

use std::sync::Arc;
use vulkano::{
    framebuffer::RenderPassAbstract,
    command_buffer::{AutoCommandBufferBuilder, DynamicState}
};

pub trait Drawable {
    fn draw(&self, builder: &mut AutoCommandBufferBuilder, renderpass: Arc <dyn RenderPassAbstract + Send + Sync>, dynamic: &mut DynamicState);
}
