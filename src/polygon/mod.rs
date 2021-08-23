use crate::Window;
use glium::{
    Vertex,
    VertexBuffer,
    Display,
    Program,
    Surface,
    index::{NoIndices, IndicesSource, PrimitiveType}
};

pub trait OnBoundPolygonInit: Vertex {
    fn program(dpy: &Display) -> &'static Program;
}

#[derive(Clone)]
pub struct Polygon <V> where V: OnBoundPolygonInit {
    vxs: Vec <V>
}

impl <V> Polygon <V> where V: OnBoundPolygonInit {
    pub const fn new() -> Self {
        Self { vxs: Vec::new() }
    }

    #[inline]
    pub fn vertex(mut self, v: V) -> Self {
        self.vxs.push(v);
        self
    }

    pub fn bind <'a> (self, window: Window) -> BoundPolygon <'a, V> {
        BoundPolygon::new(self.vxs, window.dpy(), NoIndices(PrimitiveType::TrianglesList))
    }
}

pub struct BoundPolygon <'a, V> where V: OnBoundPolygonInit {
    pub(crate) buf: VertexBuffer <V>,
    pub(crate) program: &'static Program,
    pub(crate) indices: IndicesSource <'a>,
}

impl <'a, V> BoundPolygon <'a, V> where V: OnBoundPolygonInit {
    pub(crate) fn new <I> (vxs: Vec <V>, dpy: &Display, idx: I) -> Self where I: Into <IndicesSource <'a>> {
        Self {
            buf: VertexBuffer::new(dpy, &vxs).unwrap(),
            program: V::program(dpy),
            indices: idx.into()
        }
    }
}

impl <'a, V> crate::Drawable for BoundPolygon <'a, V> where V: OnBoundPolygonInit {
    fn draw(&self, target: &mut glium::Frame) {
        target.draw(&self.buf, self.indices.clone(), self.program, &glium::uniform!(), &Default::default()).unwrap()
    }
}
