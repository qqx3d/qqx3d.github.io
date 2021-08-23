use glium::{
    Vertex,
    VertexBuffer,
    Display,
    Program,
    Surface,
    Frame,
    index::IndicesSource,
    uniforms::Uniforms
};
use crate::Color;

pub enum BoundPolygonInterfaceAction <T> {
    Move(T),
    Set(T),
    Get(*mut T),
    Reset
}

pub trait BoundPolygonInterface <U> : Vertex {
    type Move: Default;
    type Uniform: Uniforms;

    const MOVABLE: bool;
    const COLORABLE: bool;

    fn program(dpy: &Display) -> &'static Program;

    fn act_pos(_: &mut U, value: BoundPolygonInterfaceAction <Self::Move>);

    fn act_col(_: &mut U, color: BoundPolygonInterfaceAction <Color>);

    fn uniforms(_: &U) -> Self::Uniform;
}

pub struct BoundPolygon <'a, V, U, const MV: bool, const CO: bool> where V: BoundPolygonInterface <U>, U: Default {
    buf: VertexBuffer <V>,
    indices: IndicesSource <'a>,
    uniforms: U
}

impl <'a, V, U, const MV: bool, const CO: bool> BoundPolygon <'a, V, U, MV, CO> where V: BoundPolygonInterface <U>, U: Default {
    pub(crate) fn new <I> (vxs: Vec <V>, dpy: &Display, idx: I) -> Self where I: Into <IndicesSource <'a>> {
        Self {
            buf: VertexBuffer::new(dpy, &vxs).unwrap(),
            indices: idx.into(),
            uniforms: U::default()
        }
    }
}

impl <'a, V, U, const CO: bool> BoundPolygon <'a, V, U, true, CO> where V: BoundPolygonInterface <U>, U: Default {
    #[inline]
    pub fn r#move(&mut self, value: V::Move) {
        V::act_pos(&mut self.uniforms, BoundPolygonInterfaceAction::Move(value))
    }

    #[inline]
    pub fn set_pos(&mut self, value: V::Move) {
        V::act_pos(&mut self.uniforms, BoundPolygonInterfaceAction::Set(value))
    }

    #[inline]
    //noinspection RsSelfConvention
    pub fn get_pos(&mut self) -> V::Move {
        let mut v: V::Move = Default::default();
        V::act_pos(&mut self.uniforms, BoundPolygonInterfaceAction::Get(&mut v as *mut V::Move));
        v
    }

    #[inline]
    pub fn reset_pos(&mut self) {
        V::act_pos(&mut self.uniforms, BoundPolygonInterfaceAction::Reset)
    }
}

impl <'a, V, U, const MV: bool> BoundPolygon <'a, V, U, MV, true> where V: BoundPolygonInterface <U>, U: Default {
    #[inline]
    pub fn color(&mut self, color: Color, mix: f32) {
        V::act_col(&mut self.uniforms, BoundPolygonInterfaceAction::Set(Color::rgba(color.r, color.g, color.b, Color::cv(mix))))
    }

    #[inline]
    //noinspection RsSelfConvention
    pub fn get_color(&mut self) -> Color {
        let mut v: Color = Default::default();
        V::act_col(&mut self.uniforms, BoundPolygonInterfaceAction::Get(&mut v as *mut Color));
        v
    }

    #[inline]
    pub fn reset_color(&mut self) {
        V::act_col(&mut self.uniforms, BoundPolygonInterfaceAction::Reset)
    }
}

impl <'a, V, U, const MV: bool, const CO: bool> crate::Drawable for BoundPolygon <'a, V, U, MV, CO> where V: BoundPolygonInterface <U>, U: Default {
    fn draw(&self, dpy: &Display, target: &mut Frame) {
        target.draw(&self.buf, self.indices.clone(), V::program(dpy), &V::uniforms(&self.uniforms), &Default::default()).unwrap()
    }
}
