use crate::{
    glium::{Display, Surface},
    Color,
    Vec4,
    Drawable
};

pub struct DrawTarget(glium::Frame);

impl core::convert::From <&mut Display> for DrawTarget {
    fn from(dpy: &mut Display) -> Self {
        Self { 0: dpy.draw() }
    }
}

impl DrawTarget {
    pub fn clear(mut self, color: Color) -> Self {
        let color = Vec4::<f32>::from(color);
        self.0.clear_color(color.x, color.y, color.z, color.w);
        self
    }

    #[inline]
    pub fn draw <D> (mut self, drawable: &D) -> Self where D: Drawable {
        drawable.draw(&mut self.0);
        self
    }

    #[inline]
    pub fn finish(self) {
        self.0.finish().unwrap()
    }
}
