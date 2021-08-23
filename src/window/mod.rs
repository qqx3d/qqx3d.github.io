mod target;
mod drawable;
mod builder;

use builder::WindowBuilder;

pub use target::DrawTarget;
pub use drawable::Drawable;

#[derive(Copy, Clone)]
pub struct Window(usize);

impl Window {
    #[inline]
    pub fn new() -> WindowBuilder {
        WindowBuilder::default()
    }

    #[inline]
    pub fn draw(self) -> DrawTarget {
        DrawTarget::from(self.dpy())
    }

    #[inline]
    pub fn id(self) -> usize { self.0 }
}
