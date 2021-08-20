use std::{
    sync::Arc,
    iter::ExactSizeIterator
};
use vulkano::{
    buffer::{CpuAccessibleBuffer, BufferUsage},
    memory::Content
};

pub struct CompiledPolygonBuffer <T> {
    pub cab: Arc <CpuAccessibleBuffer <[T]>>
}

impl <T> CompiledPolygonBuffer <T> where T: Content + 'static {
    // TODO: Remove `device`
    pub fn new <I> (device: std::sync::Arc <vulkano::device::Device>, iter: I) -> Self where I: ExactSizeIterator <Item = T> {
        Self {
            cab: CpuAccessibleBuffer::from_iter(
                device,
                BufferUsage::all(),
                false,
                iter
            ).unwrap()
        }
    }
}
