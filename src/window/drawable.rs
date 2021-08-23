pub trait Drawable {
    fn draw(&self, dpy: &glium::Display, target: &mut glium::Frame);
}
