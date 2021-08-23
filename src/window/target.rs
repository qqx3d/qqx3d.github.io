use crate::{
    glium::{Display, Surface, Frame},
    Color,
    Vec4,
    Drawable
};

pub struct DrawTarget {
    target: Frame,
    dpy: &'static Display
}

impl core::convert::From <&'static mut Display> for DrawTarget {
    fn from(dpy: &'static mut Display) -> Self {
        Self {
            target: dpy.draw(),
            dpy
        }
    }
}

impl DrawTarget {
    pub fn clear(mut self, color: Color) -> Self {
        let color = Vec4::<f32>::from(color);
        self.target.clear_color(color.x, color.y, color.z, color.w);
        self
    }

    #[inline]
    pub fn draw <D> (mut self, drawable: &D) -> Self where D: Drawable {
        drawable.draw(self.dpy, &mut self.target);
        self
    }

    #[inline]
    pub fn finish(self) {
        self.target.finish().unwrap()
    }
}
