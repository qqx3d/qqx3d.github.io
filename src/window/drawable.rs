pub trait Drawable {
    fn draw(&self, target: &mut glium::Frame);
}
