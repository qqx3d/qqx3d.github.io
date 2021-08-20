pub mod compiled;

use crate::{Vec3, Vec4, Color};
use compiled::CompiledPolygonBuffer;

#[derive(Default, Clone)]
pub struct Vertex {
    pos:   Vec3 <f32>,
    color: Vec4 <f32>
}

impl Vertex {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn position(mut self, pos: Vec3 <f32>) -> Self {
        self.pos = pos;
        self
    }

    #[inline]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color.into();
        self
    }
}

vulkano::impl_vertex!(Vertex, pos, color);

#[derive(Clone)]
pub struct Shape {
    vxs: Vec <Vertex>
}

impl Shape {
    #[inline]
    pub const fn new() -> Self {
        Self { vxs: Vec::new() }
    }

    #[inline]
    pub fn vertex(mut self, vertex: Vertex) -> Self {
        self.vxs.push(vertex);
        self
    }

    // TODO: Remove `device`
    pub fn build(self, device: std::sync::Arc <vulkano::device::Device>) -> CompiledPolygonBuffer <Vertex> {
        CompiledPolygonBuffer::new(device, self.vxs.iter().cloned())
    }
}

