pub(crate) mod impls;

use crate::Vec2;

pub struct WindowBuilder {
    size: Vec2 <u32>,
    title: String
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            size: Vec2::new(640, 320),
            title: String::from("qqx")
        }
    }
}

impl WindowBuilder {
    #[inline]
    pub fn size(mut self, size: Vec2 <u32>) -> Self {
        self.size = size;
        self
    }

    #[inline]
    pub fn title <S> (mut self, title: S) -> Self where S: Into <String> {
        self.title = title.into();
        self
    }

    #[inline]
    pub fn build(self) -> Window {
        unsafe { impls::build(self) }
    }
}

#[derive(Copy, Clone)]
pub struct Window(usize);

impl Window {
    pub fn new() -> WindowBuilder {
        WindowBuilder::default()
    }
}
