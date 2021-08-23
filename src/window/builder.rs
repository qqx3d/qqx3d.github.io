use crate::Vec2;
use super::Window;
use glium::{
    Display,
    glutin::{
        self,
        dpi::PhysicalSize,
        window::WindowId
    }
};

static mut WINDOWS: Vec <Display> = Vec::new();

pub struct WindowBuilder {
    size: Option <Vec2 <u32>>,
    title: String
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            size: None,
            title: String::from("qqx")
        }
    }
}

impl WindowBuilder {
    #[inline]
    pub fn size(mut self, size: Vec2 <u32>) -> Self {
        self.size = Some(size);
        self
    }

    #[inline]
    pub fn title <S> (mut self, title: S) -> Self where S: ToString {
        self.title = title.to_string();
        self
    }

    pub fn build(self) -> Window {
        let mut builder = glutin::window::WindowBuilder::new().with_title(self.title);
        if let Some(size) = self.size { builder = builder.with_inner_size(PhysicalSize::from(size)) }

        let dpy = Display::new(builder, glutin::ContextBuilder::new(), crate::Stt::eventloop()).unwrap();

        unsafe {
            WINDOWS.push(dpy);
            Window(WINDOWS.len() - 1)
        }
    }
}

impl Window {
    #[inline]
    pub(crate) fn dpy(self) -> &'static mut Display {
        unsafe { &mut WINDOWS[self.0] }
    }

    pub(crate) fn by_id(id: WindowId) -> Self {
        let mut i = 0;
        unsafe {
            while i < WINDOWS.len() {
                if WINDOWS[i].gl_window().window().id() == id { return Window(i) }
                i += 1
            }
        }
        unreachable!()
    }
}
