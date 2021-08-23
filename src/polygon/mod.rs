mod bound;

use crate::Window;
use bound::BoundPolygon;
use glium::index::{NoIndices, PrimitiveType};
use core::marker::PhantomData;

pub use bound::{BoundPolygonInterface, BoundPolygonInterfaceAction};

#[derive(Clone)]
pub struct Polygon <V, U> where V: BoundPolygonInterface <U>, U: Default {
    vxs: Vec <V>,
    _marker: PhantomData <U>
}

impl <V, U> Polygon <V, U> where V: BoundPolygonInterface <U>, U: Default {
    pub const fn new() -> Self {
        Self { vxs: Vec::new(), _marker: PhantomData }
    }

    #[inline]
    pub fn vertex(mut self, v: V) -> Self {
        self.vxs.push(v);
        self
    }

    pub fn bind <'a> (self, window: Window) -> BoundPolygon <'a, V, U, { V::MOVABLE }, { V::COLORABLE }> {
        BoundPolygon::new(self.vxs, window.dpy(), NoIndices(PrimitiveType::TrianglesList))
    }
}
